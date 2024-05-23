use serde::{Serialize,Deserialize};

///
/// EChart type : declare type OrdinalRawValue = string | number;
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrdinalRawValue{
    ///
    /// 文本
    ///
    String(String),
    ///
    /// 数字
    ///
    Number(f64)

}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoolOrString{
    ///
    /// 文本
    ///
    String(String),
    ///
    /// 布尔值
    ///
    Bool(bool)

}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrdinalData{

    Vec(Vec<OrdinalRawValue>),

    Arr(Vec<Vec<OrdinalRawValue>>)

}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrdinalNumber{

    Number(f64),

    Numbers(Vec<f64>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrdinalString{

    String(String),

    Strings(Vec<String>)
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdinalSortInfo{
    ordinal_numbers:Vec<f64>
}