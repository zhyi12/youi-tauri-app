use std::collections::HashMap;
use itertools::Itertools;
use crate::dataset::DataColumn;
use crate::filter::{JoinMode};
use crate::query::dataset::{CubeQuery, LeftJoin, QueryDataset};
use crate::query::sql::{to_table_sql_leaf_union};

impl<'a> QueryDataset<'a>{

    ///
    /// 数据集单一查询脚本
    ///
    pub fn sql_one_query(&self,cube_out_columns:&Vec<(String,String)>,global_filter_script:&str)->String {
        let opt_one_query = self.build_one_query();

        match opt_one_query {
            None => "".to_string(),
            Some(mut one_query) => {
                let one_query_columns = self.find_one_query_columns(&mut one_query,cube_out_columns);
                self.sql_cube_query(&one_query,&one_query_columns,global_filter_script)
                    .replace("df_dataset as ",&format!("df_dataset_{} as ",&self.name))
            }
        }
    }

    ///
    /// 数据集立方体数据查询
    ///
    pub fn sql_cube_query(&self,cube_query:&CubeQuery,cube_query_columns:&Vec<(String,String)>,global_filter_script:&str)->String{
        //HashMap<表名、输出列集合>
        let cube_table_column_map:HashMap<String,Vec<&DataColumn>> = self.to_column_map(cube_query_columns);

        let mut script = String::new();
        let dfs = cube_query.query_tables
            .iter()
            .map(|qt| format!("df_{} as (\n  {}\n)",&qt.name,self.sql_query_table(&qt.name, &cube_table_column_map, global_filter_script)))
            .filter(|s| !s.is_empty())
            .join(",");

        script.push_str(&dfs);

        //
        // 叶子节点的left join表的取数语句
        //
        match &cube_query.leaf_join_tables(){
            None => {}
            Some(leaf_join_tables) => {
                script.push_str(",");
                script.push_str(&leaf_join_tables.iter()
                    .map(|name| format!("df_{} as (\n  {}\n)",name,self.sql_query_table(name, &cube_table_column_map, global_filter_script)))
                    .join(",\n"));
            }
        }
        script.push_str(",");
        script.push_str(&cube_query.sql(&cube_table_column_map));

        script
    }

    ///
    /// 数据查询语句
    ///
    pub fn sql_query_table(&self, name:&str, cube_table_column_map:&HashMap<String,Vec<&'a DataColumn>>, global_filter_script:&str) ->String{
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
                            self.sql_table_union(t.name(),&union_table_names,columns,global_filter_script)
                        }else{
                            t.sql(columns,global_filter_script)
                        }
                    }
                }
            }
        }
    }

    ///
    /// 合并表 union all
    ///
    pub fn sql_table_union(&self,_name:&str,union_table_names:&Vec<String>,columns:&Vec<&'a DataColumn>,global_filter_script:&str)->String{
        let column_names = columns.iter().map(|c|c.name_clone()).collect::<Vec<String>>();
        union_table_names.iter().map(|ut_name|{
            to_table_sql_leaf_union(ut_name,&column_names,global_filter_script)
        }).join("\n union all \n  ")
    }
}

impl<'a> CubeQuery<'a>  {
    ///
    /// 立方体查询的left join语句
    ///
    pub fn sql(&self,cube_table_column_map:&HashMap<String,Vec<&DataColumn>>) -> String {
        let mut script = String::new();
        script.push_str("df_dataset as (\n");

        let left_joins = self.cube_query_left_joins();

        let first_query_table = &self.query_tables[0];

        if left_joins.is_empty(){
            script.push_str(&format!("  select * from df_{}",&first_query_table.name));
        }else{
            let join_count = left_joins.len();
            //左连续信息
            //Vec<(columns,left join on,join_where)>
            let joins = left_joins.iter()
                .zip(0..join_count)
                .map(|(join,idx)|build_join_info(idx+2,join,cube_table_column_map))
                .collect::<Vec<(Vec<String>,String,String)>>();

            //叶子表输出列
            let out_columns = first_query_table.out_columns("t1.");

            // 立方体配置的过滤处理
            let sql_cube_filter = self.sql_cube_filter(&joins);

            // 首表输出列、其他表输出列 、首表名、left join on , where
            script.push_str(&format!("  select {},{} from df_{} as t1 \n    {} \n  {}",
                                     out_columns.iter().join(","),
                                     joins.iter().map(|(c,..)|c.iter().join(",")).filter(|s|!s.is_empty()).join(","),
                                     &first_query_table.name,
                                     joins.iter().map(|(_,left_on,_)|left_on).join("\n  "),
                                     sql_cube_filter,

            ));
            script.push_str("");
        }
        script.push_str(")\n");
        script
    }

    ///
    /// 立方体体过滤脚本
    ///
    pub fn sql_cube_filter(&self,joins:&Vec<(Vec<String>,String,String)>)->String{
        let where_sql = match self.cube_filter{
            None=>"".to_string(),
            Some(cube_filter)=>{
                let mut script = match cube_filter.join_mode {
                    JoinMode::Cross=>{
                        joins.iter().map(|(_,_,join_where)|join_where).join("\n    and ")
                    }
                    _ => "".to_string()
                };
                //filter items 处理
                let tree = cube_filter.filter.to_tree();
                let sql_filter = tree.sql();

                if !sql_filter.is_empty(){
                    script.push_str(" and ");
                    script.push_str(&sql_filter);
                }
                script
            }
        };

        if where_sql.is_empty(){
            "".to_string()
        }else{
            format!(" where {} \n",where_sql)
        }
    }
}
///
/// 输出列、左连接、过滤条件
///
fn build_join_info(index:usize,join:&LeftJoin,cube_table_column_map:&HashMap<String,Vec<&DataColumn>>)->(Vec<String>,String,String){
    let left_on = join.on
        .iter()
        .map(|o|format!("t1.{} = t{}.{0}",o,index))
        .join(" and ");

    let key_where = join.on
        .iter()
        .map(|o|format!("t{}.{} is not null",index,o))
        .join(" and ");

    let columns = cube_table_column_map
        .get(&join.name)
        .unwrap()
        .iter()
        .map(|dc|{
            if join.on.contains(&dc.name_clone()){
                vec![]
            }else{
                vec![format!("t{}.{}",index,dc.name())]
            }
        }).flatten()
        .collect::<Vec<String>>();
    (
        columns,
        format!("left join df_{} as t{} \n    on {}",&join.name,index, left_on),
        key_where,
    )
}