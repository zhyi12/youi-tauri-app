
use serde::{Serialize, Deserialize};
use crate::column::Columns;
use crate::dsl::DslNode;

#[derive(Serialize, Deserialize,Debug)]
pub struct DbReader{

    uri:String,

    ///
    /// csv 文件编码
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    encode:Option<String>,

    #[serde(flatten)]
    columns_info:Columns
}

impl DslNode for DbReader{
    fn to_dsl(&self, _input_step_id: Option<&str>) -> crate::error::QueryResult<String> {
        Ok(format!("{}",""))
    }
}