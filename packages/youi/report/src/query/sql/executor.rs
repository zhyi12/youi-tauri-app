use std::collections::HashMap;
use itertools::Itertools;
use log::debug;
use rhai::Engine;
use crate::cube::Cube;
use crate::query::executor::QueryFunc;
use crate::{Param, QueryExecutor};

impl<'a> QueryExecutor<'a> {

    ///
    /// sql语句查询
    ///
    pub fn sql_query(&self,engine:&Engine,callback:&mut QueryFunc,params:&Vec<Param>)->Vec<Cube>{
        //
        let opt_global_filter_tree = self.global_filter_tree();

        let param_map:HashMap<String,&Param> = self.to_param_map(params);

        //全局过滤条件
        let global_filter_script = match opt_global_filter_tree {
            None => "".to_string(),
            Some(global_filter_tree) => global_filter_tree.sql()
        };

        debug!("global_filter_script : {global_filter_script}");

        //获取初始数据立方体，根据报表配置自动调整部分内容
        let cube_list = self.data_cube_list();

        if self.is_union_dataset(){
            self.sql_union_exec(&cube_list,engine,callback,&param_map,&global_filter_script)
        }else{
            vec![]
        }
    }

    ///
    /// 数据集合并查询
    ///
    fn sql_union_exec(&self,cube_list:&Vec<Cube>,engine:&Engine,callback:&mut QueryFunc,
                      param_map:&HashMap<String,&Param>,global_filter_script:&str)->Vec<Cube>{
        let union_cube = self.exec_cube(engine,&cube_list[0],callback,param_map,|e|{
            e.global_filter_script = global_filter_script.to_string();
            let mut script = String::new();
            //数据集单一查询脚本
            let one_script = self.query_dataset()
                .iter()
                .map(|ds|ds.sql_one_query(&e.cube_out_columns,&global_filter_script))
                .join(",");

            //read汇总sql
            script.push_str("read_sql(\"${dbConnect}\",\"");

            script.push_str("with ");
            script.push_str(&one_script);
            //生成多数据集 union all 语句
            script.push_str(",df_dataset as (");
            script.push_str(&self.query_dataset()
                .iter()
                .map(|ds|format!("select * from df_dataset_{}",ds.name))
                .join("\n    union all    \n"));
            script.push_str(")");
            //数据库汇总语句
            //self.model.dataset_union_mapping();
            script.push_str(&e.cube.sql(engine,&self.model.find_dataset_union_group_select()));

            //结束read_sql
            script.push_str("\")");
            script.replace("\n","")
        });
        vec![union_cube]
    }

}