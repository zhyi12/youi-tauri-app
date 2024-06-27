use serde::{Serialize, Deserialize};
use crate::DataType;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column{
    ///
    /// 列名
    ///
    pub(crate) name:String,
    ///
    /// 数据类型
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data_type:Option<String>,
    ///
    ///
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    text:Option<String>,
    ///
    /// 别名
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    alias:Option<String>,

    ///
    /// 值
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    value:Option<DataType>,
    ///
    /// 表达式
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) expression:Option<String>,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Columns{

    ///
    /// 中文描述
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    table_title:Option<String>,
    ///
    /// 列集合
    ///
    pub(crate) columns:Vec<Column>,
    ///
    /// 选择的列名集合 selectedColumnNames
    ///
    selected_column_names:Vec<String>,

}

impl Columns {
    ///
    ///
    ///
    pub fn find_selected_columns(&self)->Vec<&Column>{
        self.columns
            .iter()
            .filter(|c|self.selected_column_names.contains(&c.name))
            .collect::<Vec<&Column>>()
    }
}