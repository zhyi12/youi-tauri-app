use serde::{Serialize, Deserialize};
use crate::dimension::{IGroup};
use crate::item::Item;
///
/// 数据集和并列维度
///
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct UnionGroup{
    pub(crate) items:Vec<Item>
}

impl UnionGroup {

    pub fn items_clone(&self)->Vec<Item>{
        self.items.clone()
    }
}

impl IGroup for UnionGroup {
    fn items(&self) -> &Vec<Item> {
        &self.items
    }

    fn table_name(&self) -> &str {
        ""
    }

    fn column_name(&self) -> &str {
        "union_col"
    }

    fn column_expression(&self) -> Option<&String> {
        None
    }

    fn set_table_name(&mut self, _table_name: String) {

    }

    fn add_items(&mut self, items: Vec<Item>) {
        self.items.extend(items)
    }
}