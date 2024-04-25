use serde::{Deserialize, Serialize};
use crate::item::ItemComponent;

///
/// 表格计算行、列
///
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct GridCalculateItem{
    #[serde(flatten)]
    item:ItemComponent,

    ///
    /// 行列表达式
    ///
    expression:Option<String>,
}