use serde::{Serialize, Deserialize};
use crate::item::Item;

///
/// 值显示维度（值显示维度和计量维度同属一个区域）
/// 1、排名、占比
/// 2、冗余存储的时间框架相关列（上年同期、上期值）
/// 3、采集组合指标的对应的分组（列名前缀约定规则）
/// 4、
///
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct ValueShow{
    ///
    /// 值显示项集合
    ///
    pub(crate) items:Vec<Item>
}

impl ValueShow {

    pub fn add_items(&mut self, items: Vec<Item>) {
        self.items.extend(items);
    }

}