use std::collections::HashMap;
use itertools::Itertools;
use log::debug;
use crate::dataset::DataColumn;
use crate::query::dataset::{CubeQuery, QueryDataset, QueryTable};

impl<'a> QueryDataset<'a>{

    ///
    /// 整个数据集生成单一的查询脚本
    ///
    pub fn dsl_one_query(&self,cube_out_columns:&Vec<(String,String)>,global_filter_script:&str)->String{
        let opt_one_query = self.build_one_query();

        match opt_one_query {
            None => "".to_string(),
            Some(mut one_query) => {
                let one_query_columns = self.find_one_query_columns(&mut one_query,cube_out_columns);
                debug!("one_query_columns {:?}",one_query_columns);
                self.dsl_cube_query(&one_query,&one_query_columns,global_filter_script)
                    .replace("let df_dataset",&format!("let df_dataset_{}",&self.name))
            }
        }
    }

    ///
    ///
    ///
    pub(crate) fn dsl_cube_query(&self,cube_query:&CubeQuery,cube_table_columns:&Vec<(String,String)>,global_filter_script:&str)->String{
        debug!("cube_table_columns:{:?}",cube_table_columns);
        //HashMap<表名、输出列集合>
        let cube_table_column_map:HashMap<String,Vec<&DataColumn>> = self.to_column_map(cube_table_columns);

        let mut script = String::new();
        //原始数据表取数语句
        let dfs = cube_query.query_tables
            .iter()
            .map(|qt| self.dsl_query_table(&qt.name, &cube_table_column_map, global_filter_script))
            .filter(|s| !s.is_empty())
            .join("\n");

        script.push_str(&dfs);
        script.push_str("\n");

        //
        // 叶子节点的left join表的取数语句
        //
        match &cube_query.leaf_join_tables(){
            None => {}
            Some(leaf_join_tables) => {
                script.push_str(&leaf_join_tables.iter().map(|name|
                    self.dsl_query_table(name, &cube_table_column_map, global_filter_script)).join("\n"));
                script.push_str("\n");
            }
        }

        //立方体查询语句
        script.push_str(&cube_query.dsl());
        script.push_str("\n");

        script
    }

    ///
    /// 单表数据查询脚本
    ///
    pub fn dsl_query_table(&self, name:&str, cube_table_column_map:&HashMap<String,Vec<&'a DataColumn>>, global_filter_script:&str) ->String{
        let opt_data_table = self.table_map.get(name);
        match opt_data_table {
            None => "".to_string(),
            Some(t) => {
                let opt_columns = cube_table_column_map.get(name);
                match  opt_columns{
                    None => "".to_string(),
                    Some(columns)=>{
                        let union_table_names = vec![vec![t.name_clone()],t.union_table_names()].concat();
                        if union_table_names.len() > 1 {
                            self.dsl_table_union(t.name(),&union_table_names,columns,global_filter_script)
                        }else{
                            //补充主键列信息
                            let key_columns = t.columns.iter()
                                .filter(|c|c.is_primary_key())
                                .collect::<Vec<&DataColumn>>();
                            let columns = vec![key_columns,columns.clone()].concat();
                            t.dsl(&columns,global_filter_script)
                        }
                    }
                }
            }
        }
    }

    ///
    /// 数据表合并
    ///
    fn dsl_table_union(&self, name:&str,union_table_names:&Vec<String>,columns:&Vec<&'a DataColumn>,global_filter_script:&str)->String{
        format!("let df_{} = concat([{}]);",name,union_table_names.iter().map(|table_name|{
            let opt_data_table = self.table_map.get(table_name);
            match opt_data_table {
                None => "".to_string(),
                Some(t) => t.dsl_reader(columns,global_filter_script,
                                        &format!("lit(\"{}\").alias(\"leaf_union_col\")",table_name))
            }
        }).filter(|s|!s.is_empty())
            .join(",\n"))
    }
    ///
    /// 生成查询的dsl脚本
    /// table_names 使用的表
    /// table_columns (表名,列名)
    ///
    pub fn dsl(&self,table_names:&Vec<String>,cube_table_columns:&Vec<(String,String)>,global_filter_script:&str)->String{
        let opt_cube_query = self.build_cube_query(table_names,&vec![],self.find_cube_filter(table_names));
        match opt_cube_query {
            None => "".to_string(),
            Some(cube_query) => {
                self.dsl_cube_query(&cube_query,cube_table_columns,global_filter_script)
            }
        }
    }

}


impl<'a> CubeQuery<'a> {

    ///
    ///
    ///
    pub fn dsl(&self)->String{
        let mut script = String::from("let df_dataset = ");
        let count = self.query_tables.len();
        if count > 1 {
            //多表join
            let join_script = (0..count-1).map(|idx|{
                let query_table = &self.query_tables[idx];
                let join_table = &self.query_tables[idx+1];
                let start = if idx == 0 {format!("df_{}",query_table.name)} else {"".to_string()};
                let left_on = join_table.primary_keys.iter().join(",");
                //join同级从表（用于使用同级从表条件或者数据集Union的模式）
                let leaf_join = if idx == 0{
                    self.dsl_leaf_join(query_table,&left_on)
                }else{
                    "".to_string()
                };

                format!("{}{}\n    .left_join(df_{},\"{}\",\"{3}\")",
                        start,
                        leaf_join,
                        join_table.name,
                        left_on)
            }).join("\n");

            script.push_str(&join_script);

            //过滤条件
            script.push_str(&self.dsl_filter());
        }else{
            //单表
            script.push_str("df_");
            script.push_str(&self.query_tables[0].name);
        }
        script.push_str(";");
        script
    }

    ///
    /// 同级从表关联语句
    ///
    fn dsl_leaf_join(&self,query_table:&QueryTable,left_on:&str)->String{
        debug!("query_table : {:?}",query_table);
        match &query_table.join_tables {
            None=>"".to_string(),
            Some(leaf_join_table)=>{
                debug!("{} leaf_join_table {:?}",&query_table.name,leaf_join_table);
                leaf_join_table.iter().map(|name|{
                    format!("\n    .left_join(df_{},\"{}\",\"{1}\")",name,left_on)
                }).join("\n    ")
            }
        }
    }
    ///
    /// 立方体多表left join 后的过滤条件
    ///
    fn dsl_filter(&self)->String{
        let mut script = String::new();
        let primary_keys = &self.query_tables[0].primary_keys;

        script.push_str("\n    .filter(");
        // 主键不为空过滤
        script.push_str(&primary_keys.iter().map(|key|{
            format!("col(\"{}\").is_not_null()",key)
        }).join(" && "));
        script.push_str(")");
        //TODO 加入立方体路径条件过滤

        script
    }
}
