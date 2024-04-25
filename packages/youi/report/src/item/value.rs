use serde::{Deserialize, Serialize};

///
/// 值显示项
///
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq,PartialOrd,Eq)]
pub struct ValueItem{

    id:String,
    ///
    /// ranking、proportion
    ///
    name:Option<String>,

    text:Option<String>,
    ///
    /// 对应的计量
    ///
    measure_item_id:Option<String>,

    ///
    /// 按显示（排名、占比）的分组
    ///
    over_group_id:Option<String>,
    ///
    /// 表达式
    /// (col(\"{name}\") - col(\"{name}_001\"))/col(\"{name}_001\")*100
    ///
    ///
    expression:Option<String>
}