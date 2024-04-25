mod reader;
mod join;
mod union;
mod select;
mod with_columns;
mod agg;
mod stats;

use itertools::Itertools;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::node::agg::Agg;
use crate::node::join::Join;
use crate::node::reader::csv::CsvReader;
use crate::node::stats::FlatCatalogTableTransform;
use crate::node::union::Union;
use crate::node::with_columns::WithColumns;
use crate::NodeInput;

///
/// 计算列
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalculateColumn{
    name:String,
    data_type:Option<String>,
    expression:String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnMapping{
    name:String,
    data_type:String,
    ///
    ///
    ///
    text:Option<String>,
    ///
    /// 别名
    ///
    alias:Option<String>,
    value:Option<Value>
}

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Node{

    Start,

    ///
    ///
    ///
    CsvReader(CsvReader),

    ///
    ///
    ///
    Join(Join),
    ///
    ///
    ///
    Union(Union),

    ///
    /// 汇总
    ///
    Agg(Agg),

    ///
    /// 新增字段
    ///
    WithColumns(WithColumns),

    ///
    /// 定长目录表列转行
    ///
    StatsFctTransform(FlatCatalogTableTransform),

    End,
}

pub trait DslNode{

    fn dsl(&self)->Option<String>;

    ///
    /// 列映射
    ///
    fn column_mapping_dsl(&self,column_mapping:&Vec<ColumnMapping>)->String{
        format!(".with_columns([{}])",column_mapping.iter()
            .map(|mapping| match &mapping.value {
                    None => match mapping.data_type.as_str() {
                        "string"|"text"=>format!("col(\"{}\").cast_str()",mapping.name),
                        "number"|"float"=>format!("col(\"{}\").cast_f64()",mapping.name),
                        "int"=>format!("col(\"{}\").cast_i64()",mapping.name),
                        _=>"".to_string()
                    },
                    Some(v) => match v {
                        Value::Number(n) => format!("lit({}).alias(\"{}\")",n,mapping.name),
                        Value::String(s) => format!("lit(\"{}\").alias(\"{}\")",s,mapping.name),
                        _=>"".to_string()
                    }
                }
            ).filter(|s|!s.is_empty())
            .join(","))
    }

}

///
/// 有输入的节点
///
pub trait InputtingAble{
    ///
    /// 设置输入
    ///
    fn set_inputs(&mut self,inputs:Vec<NodeInput>);
}

impl Node {

    pub fn set_inputs(&mut self,inputs:Vec<NodeInput>){
        match self {
            Node::Start => {}
            Node::CsvReader(_) => {}
            Node::Join(x )=>x.set_inputs(inputs),
            Node::Union(x) => x.set_inputs(inputs),
            Node::WithColumns(x) => x.set_inputs(inputs),
            Node::Agg(x) => x.set_inputs(inputs),
            Node::StatsFctTransform(x) =>x.set_inputs(inputs),
            Node::End => {}
        }
    }

    pub fn set_id(&mut self,id:String){
        match self {
            Node::Start => {}
            Node::CsvReader(x) => x.set_id(id),
            Node::Join(x) => {x.set_id(id)}
            Node::Union(x) => {x.set_id(id)}
            Node::WithColumns(x) => {x.set_id(id)}
            Node::Agg(x) => x.set_id(id),
            Node::StatsFctTransform(x) => x.id = Some(id),
            Node::End => {}
        }
    }

}

impl Node {

    ///
    ///
    ///
    pub fn dsl(&self)->Option<String>{
        match self {
            Node::Start => None,
            Node::CsvReader(x) =>x.dsl(),
            Node::Join(x) => x.dsl(),
            Node::Union(x) => x.dsl(),
            Node::WithColumns(x) => x.dsl(),
            Node::Agg(x) => x.dsl(),
            Node::StatsFctTransform(x)=>x.dsl(),
            Node::End => None,
        }
    }
}