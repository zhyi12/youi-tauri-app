use itertools::Itertools;
use log::debug;
use crate::dataset::{DataTable, DataReader, DataColumn};
use crate::DataType;

///
/// 数据读取dsl脚本相关方法
///
impl DataReader {
    ///
    ///
    ///
    pub fn dsl(&self,columns:&Vec<&DataColumn>)->String{

        let other_params = match &self.reader_params(){
            None => "".to_string(),
            Some(reader_params) => reader_params.iter().map(|d|{
                let value = match d {
                    DataType::Int(x) => x.to_string(),
                    DataType::Float(x) => x.to_string(),
                    DataType::String(x) => format!("\"{}\"",x),
                    DataType::Bool(x) => x.to_string(),
                    _=>"\"\"".to_string()
                };
                format!(",{}",value)
            }).join("")
        };

        let fields_param = if self.reader() == "read_csv"{
            format!(",[{}]",columns.iter().map(|c|format!("field(\"{}\",\"{}\")",c.name(),match &c.data_type {
                None => "".to_string(),
                Some(data_type) => {
                    data_type.to_string()
                }
            })).join(","))
        }else{
            "".to_string()
        };

        format!("{}(\"{}\"{}{})",
                self.reader(),
                self.uri(),
                fields_param,
                other_params)
    }
}

impl DataTable {

    ///
    /// 取表数据脚本
    ///
    pub fn dsl_reader(&self,columns:&Vec<&DataColumn>,global_filter_script:&str,union_expr:&str)->String{
        let mut select_columns = self.primary_keys();

        let other_columns = columns.iter()
            .filter(|column|!select_columns.contains(&column.name().to_string()))
            .map(|column|column.name_clone())
            .sorted()
            .dedup()
            .collect::<Vec<String>>();

        select_columns.extend(other_columns);

        //数据表查询输出字段
        let mut select_expr = select_columns
            .iter()
            .map(|name|format!("col(\"{}\")",name))
            .join(",");

        if !union_expr.is_empty(){
            select_expr.push_str(",");
            select_expr.push_str(union_expr);
        }

        //数据表过滤条件
        let filter_expr = build_filter_expr(global_filter_script);

        format!("{}{}\n    .select([{}])",self.reader().dsl(&columns),filter_expr,select_expr)
    }
    ///
    ///
    ///
    pub fn dsl(&self,columns:&Vec<&DataColumn>,global_filter_script:&str)->String{
        format!("let df_{}={};",self.name(),self.dsl_reader(columns,global_filter_script,""))
    }
}

///
/// 公共条件
/// 当前表条件、数据集根条件
///
fn build_filter_expr(global_filter_script:&str)->String{
    if !global_filter_script.is_empty(){
        format!("\n    .filter({})",global_filter_script)
    }else {
        "".to_string()
    }
}
