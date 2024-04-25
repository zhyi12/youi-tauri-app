
use serde::{Deserialize,Serialize};
use crate::item::Item;

///
/// 交叉表组件
///
#[derive(Serialize, Deserialize,Debug)]
pub struct CrossWidget{

    ///
    /// 头项集合
    ///
    header_items:Option<Vec<Item>>,
}