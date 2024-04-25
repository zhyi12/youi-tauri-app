use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use crate::DataType;

///
/// 立方体数据
///
#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataMap{
    ///
    /// <立方体交叉点key,值>
    ///
    data_map:HashMap<String,DataType>
}