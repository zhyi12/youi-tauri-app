use std::collections::HashMap;
use itertools::Itertools;
use rhai::Engine;
use crate::cube::Cube;
use crate::{Param, QueryExecutor};
use crate::query::dsl::constants::DF_CUBE;
use crate::query::executor::QueryFunc;

impl<'a> QueryExecutor<'a> {

    ///
    /// dsl 查询
    ///
    pub fn dsl_query(&self,engine:&Engine,callback:&mut QueryFunc,params:&Vec<Param>)->Vec<Cube>{

        let opt_global_filter_tree = self.global_filter_tree();
        let param_map:HashMap<String,&Param> = self.to_param_map(params);
        //全局过滤条件
        let global_filter_script = match opt_global_filter_tree {
            None => "".to_string(),
            Some(global_filter_tree) => global_filter_tree.dsl()
        };
        //获取初始数据立方体，根据报表配置自动调整部分内容
        let cube_list = self.data_cube_list();

        if self.is_union_dataset(){
            //1、从数据集生成union语句
            //2、基于union集的立方体查询语句
            let union_cube = self.exec_cube(engine,&cube_list[0],callback,&param_map,|e|{
                e.global_filter_script = global_filter_script.to_string();

                let mut script = String::new();
                //数据集单一查询脚本
                let one_script = self.query_dataset()
                    .iter()
                    .map(|ds|ds.dsl_one_query(&e.cube_out_columns,&global_filter_script))
                    .join("");

                script.push_str(&one_script);
                //生成 union语句
                script.push_str(&format!("let {} = concat([{}]);\n",
                                         DF_CUBE,
                                         self.query_dataset()
                                             .iter()
                                             .zip(0..self.query_dataset().len())
                                             .map(|(q,idx)|format!("\n    {}_{}.with_columns([lit(\"{}\").alias(\"union_col\")])",DF_CUBE,q.name(),idx))
                                             .join(",")
                ));
                //生成 union 后的汇总语句
                script.push_str(&e.dsl_cube());
                script
            });
            vec![union_cube]
        }else{
            //并行执行,并返回带数据的立方体
            cube_list
                .iter()
                .map(|cube|self.exec_cube(engine,&cube,callback,&param_map,|e| {
                    e.global_filter_script = global_filter_script.to_string();
                    e.dsl()
                }))
                .collect::<Vec<Cube>>()
        }
    }
}