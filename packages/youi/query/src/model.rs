use serde::{Serialize,Deserialize};
use crate::step::Step;

///
/// 查询模型
///
#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryModel{
    ///
    /// 唯一标志
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    id:Option<String>,
    ///
    /// 查询名称
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    text:Option<String>,
    ///
    /// 步骤
    ///
    pub(crate) steps:Vec<Step>,

    ///
    /// 列信息
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) column_infos:Option<Vec<ColumnInfo>>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ColumnInfo{
    ///
    ///
    ///
    name:String,
    ///
    /// 列宽度
    ///
    width:Option<i32>
}