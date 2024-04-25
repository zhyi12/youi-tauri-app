use serde::{Serialize,Deserialize};
use crate::DataType;

///
/// 当一张表存在多个数据集时，可配置快速合并规则
/// 当存在UnionMapping时，
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataUnion{
    ///
    ///
    ///
    id:Option<String>,
    ///
    /// Union的显示名称，默认显示合并
    ///
    text:Option<String>,
    ///
    /// 数据集生成列值映射，默认使用表名称
    ///
    pub(crate) mapping:Vec<UnionMapping>
}

///
///
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnionMapping{
    ///
    /// 显示文本
    ///
    pub(crate) text:String,
    ///
    /// 值表达式
    ///
    pub(crate) value_expression:Option<String>,
    ///
    /// 值
    ///
    pub(crate) value:Option<DataType>
}

impl UnionMapping {

    ///
    /// union列表达式
    ///
    pub fn expression(&self,ref_column_name:&str)->String{
        match &self.value_expression {
            None => format!("col(\"{}\").eq({})",ref_column_name,self.value()),
            Some(value_expression) => value_expression.to_string()
        }
    }
    ///
    /// 值
    ///
    pub fn value(&self)->String{
        match &self.value {
            None => "".to_string(),
            Some(value) => match value {
                DataType::Int(x) => x.to_string(),
                DataType::Float(x) => x.to_string(),
                DataType::String(x) => format!("\"{}\"",x),
                _ => "".to_string()
            }
        }
    }
}