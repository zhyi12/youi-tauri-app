use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::OrdinalRawValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortConfig{

    ///
    /// 单一排序
    ///
    One(HashMap<String,OrdinalRawValue>),

    ///
    /// 多维度排序
    ///
    Mul(Vec<HashMap<String,OrdinalRawValue>>)

}