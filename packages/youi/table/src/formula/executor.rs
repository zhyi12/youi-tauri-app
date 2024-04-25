use hashbrown::{HashSet, HashMap};
use grid::{Grid};
use log::{debug,error};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use regex::{Regex};
use rhai::{Dynamic, Engine, Scope};
use youi_calamine::DataType;
use youi_flow::{Flow};
use youi_flow::node::Node;
use youi_flow::transition::Transition;
use crate::{Cell, ColumnId, func_area_regex, func_get_regex, TableList};
use crate::formula::formula::{Formula, formula_id};
use crate::formula::position::{FuncAreaRowExpandReplacer, FuncGetRowExpandReplacer,FuncGetColExpandReplacer,FuncAreaColExpandReplacer};
use crate::formula::transform::transform_expression;

///
///
///
pub struct FormulaExecutor{
    ///
    /// 公式集合
    ///
    formula_list:Vec<Formula>,
    ///
    ///
    ///
    table_list:TableList,
    ///
    /// 公式执行流程
    ///
    flow:Option<Flow>
}

impl FormulaExecutor {

    pub fn new(table_list:TableList)->Self{
        let formula_list = table_list.find_formula_list();
        Self{
            table_list,
            formula_list,
            flow:None,
        }
    }
}

impl FormulaExecutor {

    ///
    /// 报表执行
    ///
    pub fn execute(&mut self,engine:&Engine) -> &TableList{
        let start = std::time::SystemTime::now();
        //表达式预处理
        self.prepare_formulas(&engine);
        debug!("表达式预处理：{:?}毫秒", std::time::SystemTime::now().duration_since(start).unwrap().as_millis());
        //构建有向无环图
        self.fill_flow();
        debug!("构建有向无环图：{:?}毫秒", std::time::SystemTime::now().duration_since(start).unwrap().as_millis());
        let opt_batch_indexes = self.find_batch_index();

        let mut data_map = self.table_list.find_data_map();

        opt_batch_indexes.iter().for_each(|indexes|{
            println!("执行批次,{}个表达式.",indexes.len());
            let scope = build_scope(&data_map);
            let result = indexes.into_par_iter().map(|idx|{
                let f = &self.formula_list[idx-1];
                let batch_scope = scope.clone();
                exec_cell_formula(f,engine,&batch_scope)
            }).filter(|s|s.is_some())
                .collect::<Vec<Option<(String,(u32,u32),DataType)>>>();
            //写入计算后的数据
            result.iter().for_each(|opt|{
                match opt {
                    Some((name,(row,col),value))=>{
                        //
                        let sheet_name  = self.table_list.find_table_names().get(name).unwrap();
                        data_map.get_mut(sheet_name).unwrap().get_mut(row-1,col-1).unwrap().text = Some(value.clone());
                    }
                    None=>{}
                }
            });
        });

        //更新table list
        self.table_list.update(&data_map);

        &self.table_list
    }
    ///
    /// 表达式预处理
    ///
    pub fn prepare_formulas(&mut self,engine:&Engine){
        let count = self.formula_list.len();

        let exec_expression_list = (&self.formula_list).into_par_iter().map(|f|{
            if f.expression.is_empty(){
                "".to_string()
            }else {
                transform_expression(&f.sheet_name,&f.expression,engine,self.table_list.find_table_names())
            }
        }).collect::<Vec<String>>();

        //转换执行公式
        (0..count).for_each(|idx|{
            self.formula_list.get_mut(idx).unwrap().set_exec_expression(&exec_expression_list[idx]);
        });

        //扩展区域公式处理
        let area_regex = Regex::new("(?<col_start>[A-Z]+)(?<row_start>[0-9]+):(?<col_end>[A-Z]+)(?<row_end>[0-9]+)").unwrap();
        let get_func_regex = func_get_regex();
        let area_func_regex = func_area_regex();

        let expanded_formulas = (&self.formula_list).into_par_iter()
            .filter(|f| f.ref_area.is_some() && f.exec_expression.is_some() && !f.exec_expression.as_ref().unwrap().is_empty())
            .map(|f|{
                match  (&f.ref_area,&f.exec_expression){
                    (Some(ref_area),Some(exec_expression))=>{
                        parse_ref_area(ref_area,&f.sheet_name,exec_expression,&area_regex,&get_func_regex,&area_func_regex)
                    }
                    _=>vec![]
                }
            }).flatten().collect::<Vec<(String,String)>>();
        //写入扩展公式
        if !expanded_formulas.is_empty(){
            let expanded_formulas: HashMap<String, String> = expanded_formulas.iter().cloned().collect();
            self.formula_list.par_iter_mut().for_each(|f|{
                if (f.exec_expression.is_none() || f.exec_expression.as_ref().unwrap().is_empty()) && expanded_formulas.contains_key(&f.id){
                   f.exec_expression = Some(expanded_formulas.get(&f.id).unwrap().to_string())
                }
            });
        }
    }

    ///
    /// 把有公式单元格构建有向无环图
    ///
    pub fn fill_flow(&mut self){
        let regex_area:Regex = Regex::new("T_(?<name>[\\w|[0-9]]+),AREA\\((?<row_start>[0-9]+),(?<row_end>[0-9]+),(?<col_start>[0-9]+),(?<col_end>[0-9]+)\\)").unwrap();
        let regex_cell:Regex = Regex::new("T_(?<name>[\\w|[0-9]]+).GET\\((?<row_index>[0-9]+),(?<col_index>[0-9]+)\\)").unwrap();

        println!("{:?}",self.table_list.find_table_names());
        //公式
        let nodes = to_node_list(&self.formula_list,self.table_list.find_table_names());
        //依赖关系
        let transitions = to_transition_list(&nodes,&self.formula_list,self.table_list.find_table_names(),&regex_area,&regex_cell);
        // //构建flow
        self.flow = Some(Flow::new(vec![vec![Node::new("start")],nodes].concat(),transitions));
    }
    ///
    ///
    ///
    pub fn find_batch_index(&self)->Vec<Vec<usize>>{
        match  &self.flow{
            Some(flow)=>{
                let mut visitor = crate::formula::visitor::FormulaFlowVisitor::new();
                flow.visit(&mut visitor);
                visitor.find_batch_indexes()
            }
            _ => vec![]
        }
    }

}

///
/// area_expr 当前单元格扩展到的区域表达式 B23
/// index 当前单元格的定位，从1开始
/// exec_expression 当前单元格的执行公式
/// area_regex area_expr的正则匹配
///
fn parse_ref_area(area_expr:&str,table_name:&str,exec_expression:&str,
                  area_regex:&Regex,get_func_regex:&Regex,area_func_regex:&Regex)->Vec<(String,String)>{
    //
    let opt_caps = area_regex.captures_at(area_expr,0);

    match opt_caps {
        Some(caps) => {
            let row_start = (&caps["row_start"]).parse::<u32>().unwrap();
            let row_end = (&caps["row_end"]).parse::<u32>().unwrap();
            let col_start =  ColumnId::from(&caps["col_start"]).index();
            let col_end = ColumnId::from(&caps["col_end"]).index();

            let (row_expanded_formulas,col_expanded_formulas) = rayon::join(||{
                //行扩展
                expand_row_formula(table_name,exec_expression,row_start,row_end,col_start,area_func_regex,get_func_regex)
            },||{
                //列扩展
                expand_col_formula(table_name,exec_expression,col_start,col_end,row_start,area_func_regex,get_func_regex)
            });

            let mut expanded_formulas = Vec::from_iter(row_expanded_formulas);
            expanded_formulas.extend(col_expanded_formulas);

            expanded_formulas
        },
        None=>vec![]
    }
}

///
/// 展开excel的行扩展公式
///
fn expand_row_formula(table_name:&str,exec_expression:&str,start:u32,end:u32,col_index:u32,area_func_regex:&Regex,get_func_regex:&Regex)->Vec<(String,String)>{
    if start < end{
        //相关区域或者单元格的行偏移，从row_start+1 开始
        (start..end).map(|idx|{
            let offset = idx - start + 1;
            //行号替换
            let func_get_replaced = get_func_regex
                .replace_all(exec_expression,FuncGetRowExpandReplacer{ offset }).to_string();
            (formula_id(table_name,(start+offset,col_index)),area_func_regex.replace_all(&func_get_replaced,FuncAreaRowExpandReplacer{offset}).to_string())
        }).collect::<Vec<(String,String)>>()
    }else{
        vec![]
    }
}
///
/// 展开excel的列扩展公式
///
fn expand_col_formula(table_name:&str,exec_expression:&str,start:u32,end:u32,row_index:u32,area_func_regex:&Regex,get_func_regex:&Regex)->Vec<(String,String)>{
    if start < end{
        //相关区域或者单元格的行偏移，从row_start+1 开始
        (start..end).map(|idx|{
            let offset = idx - start + 1;
            //行号替换
            let func_get_replaced = get_func_regex
                .replace_all(exec_expression,FuncGetColExpandReplacer{ offset }).to_string();
            (formula_id(table_name,(row_index,start+offset)),area_func_regex.replace_all(&func_get_replaced,FuncAreaColExpandReplacer{offset}).to_string())
        }).collect::<Vec<(String,String)>>()
    }else{
        vec![]
    }
}

///
///
///
// fn build_script(indexes:&Vec<usize>,formula_list:&Vec<Formula>,table_names:&HashMap<String,String>)->String{
//
//     let mut script = indexes.iter().map(|idx|{
//         let formula = &formula_list[*idx];
//         match &formula.exec_expression {
//             None => "".to_string(),
//             Some(exec_expression) => {
//                 if exec_expression.is_empty(){
//                     "".to_string()
//                 }else{
//                     let table_name = table_names.get(&formula.sheet_name).unwrap();
//                     format!("SET(T_{},{},{},{})",table_name,formula.index.0-1,formula.index.1-1,exec_expression)
//                 }
//             }
//         }
//     }).filter(|s|!s.is_empty()).join(";\n");
//
//     script.push_str(";return [");
//     script.push_str("T_");
//     script.push_str(&table_names.values().sorted().join(",T_"));
//     script.push_str("];");
//
//     script
//
// }
// ///
// ///
// ///
fn exec_cell_formula(formula:&Formula, engine:&Engine, batch_scope:&Scope) ->Option<(String, (u32, u32), DataType)>{
    match &formula.exec_expression{
        None => None,
        Some(exec_expression) =>{
            if exec_expression.is_empty(){
                None
            }else {
                let mut exec_scope = batch_scope.clone();

                match engine.eval_expression_with_scope::<Dynamic>(&mut exec_scope,&exec_expression){
                    Ok(result)=>{
                        //
                        let cell_value = if result.is_float(){
                            DataType::Float(result.clone_cast::<f64>())
                        } else if result.is_int(){
                            DataType::Int(result.clone_cast::<i64>())
                        }else if result.is_string(){
                            DataType::String(result.clone_cast::<String>())
                        }else{
                            let opt_value = result.try_cast::<DataType>();
                            match opt_value {
                                Some(value)=>value.clone(),
                                None=>DataType::Empty
                            }
                        };
                        // debug!("{exec_expression} calculate value: {:?}",cell_value);
                        Some((formula.sheet_name.clone(),formula.index,cell_value))
                    }
                    Err(e)=>{
                        error!("{} => {exec_expression} error {}",formula.expression,e);
                        None
                    }
                }
            }
        }
    }
}
///
/// 脚本Scope
///
fn build_scope(datas:&HashMap<String,Grid<Cell>>)->Scope{
    let mut scope = Scope::new();
    datas.iter().for_each(|(k,g)|{
        scope.push(format!("T_{}",k),g.clone());
    });
    scope
}

///
/// 节点
///
fn to_node_list(formula_list:&Vec<Formula>, table_names: &HashMap<String, String>) ->Vec<Node>{
    (formula_list).iter().map(|f|{
        Node::new(&formula_id(table_names.get(&f.sheet_name).unwrap(),f.index))
    }).collect::<Vec<Node>>()
}

///
/// 依赖关系
///
fn to_transition_list(nodes:&Vec<Node>, formula_list:&Vec<Formula>, table_names: &HashMap<String, String>, regex_area:&Regex, regex_cell:&Regex) ->Vec<Transition>{
    let key_set:HashSet<String> = HashSet::from_iter(nodes.into_par_iter().map(|node|node.id.to_string()).collect::<Vec<String>>());

    let mut transition_tuple_list = (formula_list).into_par_iter().map(|f|{
        match &(f.exec_expression){
            None => vec![],
            Some(exec_expression) => {
                let area_prev_keys = regex_area.captures_iter(exec_expression).map(|cap|{
                    let sheet_name = &cap["name"];
                    let row_start = (&cap["row_start"]).parse::<u32>().unwrap();
                    let row_end = (&cap["row_end"]).parse::<u32>().unwrap();
                    let col_start = (&cap["col_start"]).parse::<u32>().unwrap();
                    let con_end = (&cap["col_end"]).parse::<u32>().unwrap();

                    (row_start..row_end+1).map(|row_index|
                        (col_start..con_end+1)
                            .map(|col_index|{
                                let id = formula_id(sheet_name,(row_index,col_index));
                                if key_set.contains(&id){
                                    Some(id)
                                }else{
                                    None
                                }
                            }).filter(|n|n.is_some())
                            .map(|s|s.unwrap()).collect::<Vec<String>>()
                    ).flatten().collect::<Vec<String>>()
                }).flatten().collect::<Vec<String>>();

                let cell_prev_keys = regex_cell.captures_iter(exec_expression).map(|cap|{
                    let sheet_name = &cap["name"];
                    let row_index = (&cap["row_index"]).parse::<u32>().unwrap();
                    let col_index = (&cap["col_index"]).parse::<u32>().unwrap();
                    let id = formula_id(sheet_name,(row_index,col_index));

                    // println!("PREV ID {id}");
                    if key_set.contains(&id){
                        Some(id)
                    }else{
                        None
                    }
                }).filter(|s|s.is_some()).map(|s|s.unwrap()).collect::<Vec<String>>();

                let mut prev_keys = vec![area_prev_keys,cell_prev_keys].concat();

                if prev_keys.is_empty(){
                    vec![]
                }else{
                    let node_id = &formula_id(table_names.get(&f.sheet_name).unwrap(),f.index);
                    prev_keys.sort();
                    prev_keys.dedup();
                    prev_keys.iter().map(move |key|(key.clone(), node_id.to_string())).collect::<Vec<(String,String)>>()
                }
            }
        }
    }).flatten().collect::<Vec<(String,String)>>();

    transition_tuple_list.sort();
    transition_tuple_list.dedup();

    //输入
    let input_ids = transition_tuple_list.iter()
        .map(|(_,v)|v.to_string())
        .collect::<Vec<String>>();

    //无输入的使用start作为输入
    let mut transitions = key_set.into_par_iter()
        .filter(|key|!input_ids.contains(key))
        .map(|key|Transition::new("start",&key))
        .collect::<Vec<Transition>>();

    transitions.extend(transition_tuple_list.iter()
        .map(|(from,to)|Transition::new(&from,&to))
        .collect::<Vec<Transition>>());

    transitions
}

