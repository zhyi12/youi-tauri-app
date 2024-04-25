use std::collections::HashMap;
use itertools::Itertools;
use rhai::Engine;
use log::debug;
use crate::cube::Cube;
use crate::dataset::{DataColumn, Node};
use crate::query::widget::{ReportWidgetExecutor, WidgetExecutor};
use crate::{Param, replace_param_value, ReportModel};
use crate::filter::FilterNode;
use crate::item::Item;
use crate::query::cube::CubeExecutor;
use crate::query::data::DataMap;
use crate::query::dataset::{ QueryDataset};
use crate::tree::Tree;
use crate::widget::Widget;

///
///
///
pub type QueryFunc = dyn FnMut(&Engine,&Cube)->Option<DataMap>+Send+Sync;

///
///
///
pub struct QueryExecutor<'a>{

    pub(crate) model: &'a ReportModel,

    ///
    /// 查询数据集
    ///
    query_dataset:Vec<QueryDataset<'a>>,

    ///
    /// 数据集全部列信息
    ///
    dataset_columns:Vec<Vec<(&'a DataColumn,&'a str)>>,

    ///
    /// 数据表、数据集对照
    ///
    table_name_mapping:HashMap<String,String>,

    ///
    /// 合并表对照
    ///
    union_mapping:HashMap<String,String>,

    ///
    ///
    ///
    widget_executors:Vec<WidgetExecutor<'a>>,

    ///
    /// 全局过滤树
    ///
    global_filter_tree:Option<Tree<FilterNode>>,
}

impl<'a> From<&'a ReportModel> for QueryExecutor<'a>{

    fn from(model: &'a ReportModel) -> Self {
        // 数据集预处理
        let query_dataset =
            model.dataset_tables().unwrap()
                .iter()
                .map(|table|QueryDataset::from(table).set_model(model))
                .collect::<Vec<QueryDataset>>();
        //数据集树集合
        let data_table_tree = query_dataset
            .iter()
            .map(|q|q.tree())
            .collect::<Vec<&Tree<Node>>>();

        // 生成数据表全组合集合
        let table_name_mapping = model.find_table_name_mapping(&data_table_tree);
        // 生成合并表对照
        let union_mapping = model.find_table_union_mapping(&data_table_tree);
        // 生成数据集列对应关系
        let dataset_columns = model.find_dataset_columns();
        // 生成组件执行
        let widget_executors = to_widget_executors(model.widgets());

        QueryExecutor{
            model,
            query_dataset,
            dataset_columns,
            table_name_mapping,
            union_mapping,
            widget_executors,
            global_filter_tree:model.global_filter_tree()
        }
    }
}

impl<'a> QueryExecutor<'a> {
    ///
    ///
    ///
    pub fn global_filter_tree(&self)->Option<&Tree<FilterNode>>{
        self.global_filter_tree.as_ref()
    }
    ///
    ///
    ///
    pub fn query_dataset(&self) -> &Vec<QueryDataset<'a>> {
        &self.query_dataset
    }
}

impl<'a> QueryExecutor<'a> {

    ///
    ///
    ///
    pub(crate) fn to_param_map<'b>(&'b self,params:&'b Vec<Param>)->HashMap<String,&Param>{
        HashMap::from_iter(
            params
                .iter()
                .map(|p|(p.name_clone(),p))
                .collect::<Vec<(String,&Param)>>()
        )
    }

    ///
    ///
    ///
    pub(crate) fn exec_cube<F>(&self,engine:&Engine,cube:&Cube,callback:&mut QueryFunc,param_map:&HashMap<String,&Param>,mut f:F)->Cube
        where
            F:FnMut(&mut CubeExecutor)->String
    {
        let mut clone_cube = cube.clone();
        //立方体所在的数据集
        let opt_dataset_name = clone_cube.find_dataset_name(&self.table_name_mapping);
        match opt_dataset_name{
            None => {},
            Some(dataset_name) => {
                //立方体数据查询模型
                let opt_cube_executor = self.build_cube_executor(dataset_name,&mut clone_cube);
                match opt_cube_executor {
                    None=>{},
                    Some(mut cube_executor)=>{
                        // 立方体查询脚本
                        let script = f(&mut cube_executor);
                        // 脚本中的参数处理
                        let script = replace_param_value(&script,param_map);

                        // 执行查询脚本（SQL语句，DSL 都转换为Polars DSL方式执行）
                        debug!("dsl script: \n{script}");
                        // 执行脚本写入立方体
                        clone_cube.set_script(script);
                        // 脚本执行
                        let opt_data_map:Option<DataMap> = callback(engine,&clone_cube);
                        match opt_data_map {
                            None => {}
                            Some(data_map) => clone_cube.set_data_map(data_map)
                        }
                        // 立方体计算
                    }
                }
            }
        };
        clone_cube
    }

    ///
    /// 提取立方体对应的数据查询模型
    ///
    fn build_cube_executor<'b>(&'b self,dataset_name:&str,cube:&'b mut Cube)->Option<CubeExecutor>{
        //立方体对应的完整数据集的数据表列集合
        let table_columns = cube.parse_table_columns();
        //当前立方体相关的表
        let cube_table_names = table_columns.iter().map(|(table_name,column_name)|{
            let opt_owner = self.table_name_mapping.get(table_name);
            match opt_owner {
                None => (table_name.to_string(),table_name.to_string()),
                Some(owner) => {
                    if owner == dataset_name {
                        (table_name.to_string(),table_name.to_string())
                    }else{
                        //模型上的分组所属数据表不在计量所在的数据集时，需要通过列名查找计量所在数据集中对应的数据表
                        //根据列名在数据集中查找数据表
                        let opt_searched_table_name =
                            self.model.find_data_table_name(&self.dataset_columns,dataset_name,column_name);
                        match opt_searched_table_name {
                            None=>(table_name.to_string(),table_name.to_string()),
                            Some(searched_table_name)=>(searched_table_name,table_name.to_string())
                        }
                    }
                }
            }
        }).collect::<Vec<(String,String)>>();

        //当前数据集外的数据表
        let table_name_mapping:HashMap<String,String> = HashMap::from_iter(cube_table_names
            .iter()
            .filter(|(table_name,outer_table_name)|{
                table_name!=outer_table_name
            })
            .sorted()
            .dedup()
            .map(|(n1,n2)|(n2.to_string(),n1.to_string()))
            .collect::<Vec<(String,String)>>());

        //当前数据集外的数据表替换为数据集内的表
        if !table_name_mapping.is_empty(){
            cube.replace_group_table_name(&table_name_mapping);
        }
        //立方体用到的数据表名集合
        let sub_table_names = cube_table_names
            .iter()
            .map(|(n1,_n2)|n1.to_string())
            .sorted()
            .dedup()
            .collect::<Vec<String>>();
        //根据数据集名查找查询用数据集
        let opt_query = self.query_dataset
            .iter()
            .find(|q|q.name() == dataset_name);

        // //立方体CubeFilter
        // let cube_filter = self.model.find_cube_filter(&sub_table_names);
        // println!("{:?} cube_filter {:?}",sub_table_names,cube_filter);

        match opt_query{
            None=>None,
            Some(query)=>{
                //立方体查询执行器
                Some(CubeExecutor::new(self.model,
                    query,
                    cube,
                    sub_table_names,
                    self.find_cube_out_columns(dataset_name,cube)
                ))
            }
        }
    }
    ///
    /// Result Vec(表名,列名)
    ///
    fn find_cube_out_columns(&self,dataset_name:&str,cube:&Cube)->Vec<(String,String)>{
        //立方体列集合 Vec<(数据表名、列名)>
        let mut table_columns = cube.parse_table_columns();
        //从列计算表达式中提取的列名
        let column_expression_columns = cube.find_column_expression_column_names();
        //合并从计算表达式提取的列名
        if !column_expression_columns.is_empty(){
            table_columns.extend(self.dataset_columns
                .iter()
                .map(|d|{
                    if !d.is_empty() && d[0].1 == dataset_name{
                        d.iter()
                            .map(|(column,table_name)|{
                                let column_name = column.name_clone();
                                if column_expression_columns.contains(&column_name){
                                    vec![(table_name.to_string(),column_name)]
                                }else {
                                    vec![]
                                }
                            })
                            .flatten()
                            .collect::<Vec<(String,String)>>()
                    }else {
                        vec![]
                    }
                })
                .sorted()
                .dedup()
                .flatten()
                .collect::<Vec<(String,String)>>())
        }
        table_columns
    }

    ///
    /// 提取报表模型的立方体
    ///
    pub fn data_cube_list(&self)->Vec<Cube>{
        let mut cubes = self.widget_executors
            .iter()
            .map(|e|e.data_cube_list(&self.union_mapping))
            .flatten()
            .collect::<Vec<Cube>>();
        if cubes.len()>1{
            //立方体合并
            self.merge_cubes(&mut cubes);
        }
        cubes
    }

    ///
    ///
    ///
    pub fn is_union_dataset(&self)->bool{
        self.model.is_union_dataset()
    }
    ///
    /// 立方体合并
    /// 1、维度完全相同，且叶子表都属于相同的表的从表的情况下，合并立方体
    ///
    ///
    fn merge_cubes(&self,cubes:&mut Vec<Cube>){
        if self.is_union_dataset(){
            //多数据集Union模式下，只支持一个立方体的汇总计算
            //合并计量项
            let merged_measure_items = cubes.iter().map(|cube|{
                match cube.find_measure_items() {
                    None => vec![],
                    Some(items) => items.clone()
                }
            }).flatten().collect::<Vec<Item>>();
            //保留第一个立方体
            cubes.truncate(1);
            //重新设置立方体的计量项为合并后的计量项目
            cubes.get_mut(0).unwrap().set_measure_items(merged_measure_items);

            debug!("cubes {}:{:?}",cubes.len(),cubes);
        }else{
            //
        }
    }

}
///
///
///
fn to_widget_executors(widgets:&Vec<Widget>)->Vec<WidgetExecutor>{
    widgets.iter().map(|w|{
        match w {
            Widget::Report(w) => WidgetExecutor::Report(ReportWidgetExecutor::from(w)),
            _ => WidgetExecutor::Empty
        }
    }).collect::<Vec<WidgetExecutor>>()
}