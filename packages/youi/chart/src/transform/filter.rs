use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::OrdinalRawValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterConfig{

    ///
    /// and｜or 连接
    ///
    Map(HashMap<String,Vec<HashMap<String,OrdinalRawValue>>>),

    ///
    /// 单一条件
    ///
    Condition(HashMap<String,OrdinalRawValue>),
    ///
    /// 单一 not条件
    ///
    ConditionalExpression(HashMap<String,HashMap<String,OrdinalRawValue>>),

}
