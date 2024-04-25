use itertools::Itertools;

mod executor;
mod filter;
mod dataset;
mod data_table;
mod cube;
mod translate;

pub use translate::SqlTranslator;

///
///
///
pub fn to_table_sql(name: &str, column_names: &Vec<String>, filter_script: &str) ->String{
    let sql_where = if filter_script.is_empty() {"".to_string()} else { format!(" where {}",filter_script) };
    format!("select {} from stats.{}{}",column_names.iter().join(","),name,sql_where)
}

///
///
///
pub fn to_table_sql_leaf_union(name: &str, column_names: &Vec<String>, filter_script: &str) ->String{
    let sql_where = if filter_script.is_empty() {"".to_string()} else { format!(" where {}",filter_script) };
    format!("select {},'{}' as leaf_union_col from  stats.{1}{}",column_names.iter().join(","),name,sql_where)
}
