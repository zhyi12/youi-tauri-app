use itertools::Itertools;
use log::debug;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rhai::Engine;
use crate::cube::Cube;
use crate::query::column::ColumnSelect;
use crate::SqlTranslator;

impl Cube {

    ///
    /// 数据汇总脚本
    ///
    pub fn sql(&self,engine:&Engine,union_group_select:&Option<ColumnSelect>) ->String{
        //计量
        let measure_select = self.find_measure_select();
        let group_select = self.find_group_select(union_group_select);

        //合并计量和分组维度输出列
        let column_select = vec![
            measure_select.iter().collect::<Vec<&ColumnSelect>>(),
            group_select.iter().collect::<Vec<&ColumnSelect>>(),
        ].concat();

        //包含表达式处理的列
        let has_select_expression =
            column_select
                .iter()
                .find(|s|s.has_value_mapping()||s.has_column_expression()).is_some();

        //立方体汇总输出
        let mut sql_cube_select = measure_select
            .iter()
            .map(|s|{
                let aggregate = &s.id[s.name.len()+1..s.id.len()];
                format!("{}({}) as {}",aggregate,s.select_name(),s.id)
            })
            .join(",");
        sql_cube_select.push_str(",");
        sql_cube_select.push_str(&group_select.iter().map(|g|g.select_name()).join(","));
        // group by
        let group_by = group_select.iter().map(|g|&g.id).join(",");

        if has_select_expression{
            // 如果存在列计算表达式，先处理列表达式映射
            let sql_column_select = column_select
                .into_par_iter()
                .map(|column|column.sql(engine))
                .collect::<Vec<String>>()
                .iter()
                .join(",");
            format!("\n  select {} from (\n    select {} from df_dataset\n) as t_ group by {}",sql_cube_select,sql_column_select,group_by)
        }else{
            //直接select column
            format!(" select {} from df_dataset group by {}",sql_cube_select,group_by)
        }
    }
}

///
///
///
impl ColumnSelect {

    ///
    /// 分组列映射
    ///
    pub fn sql(&self,engine:&Engine)->String{
        match &self.column_expression {
            None => {
                if self.has_value_mapping(){
                    let when_then_chain = self.value_mapping().into_par_iter().map(|(id,expression)|{
                        let opt_cond_sql = SqlTranslator::from(engine).sql_column_expression(expression);
                        match  opt_cond_sql{
                            Ok(cond) => format!("when {} then '{}'",cond,id),
                            Err(_e) => {
                                debug!("parse error:{expression}");
                                "".to_string()
                            }
                        }
                    }).collect::<Vec<String>>().iter().join(" ");

                    format!("case {} else '999' end as {}",when_then_chain,self.select_name())
                }else{
                    self.name.to_string()
                }
            }
            Some(column_expression) => {
                debug!("column_expression {column_expression}");
                format!("{} as {}",SqlTranslator::from(engine).sql_column_expression(column_expression).unwrap(),self.select_name())
            }
        }
    }
}