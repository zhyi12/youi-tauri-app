use serde::{Serialize, Deserialize};

///
/// 计量项
///
#[derive(Serialize, Deserialize,Debug)]
pub struct MeasureItem{

    ///
    ///
    ///
    pub(crate) name:String,

    ///
    ///
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    text:Option<String>,

    ///
    ///
    ///
    pub(crate) aggregate:String,

    ///
    /// 精度
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    precision:Option<usize>,

}