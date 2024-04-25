mod connect;
mod item;
mod cube;
mod operator;

use serde::{Serialize, Deserialize};
use crate::filter::connect::{AndConnect, OrConnect};
pub use crate::filter::item::FilterItem;
pub use crate::filter::cube::{CubeFilter,JoinMode};
pub use crate::filter::operator::FilterOperator;
use crate::tree::{LevelNode, Tree};

///
/// 过滤条件
///
#[derive(Serialize,Deserialize,Debug)]
pub struct Filter{
    ///
    ///
    ///
    items:Vec<FilterNode>
}

impl Filter {

    pub fn to_tree(&self)->Tree<FilterNode>{
        Tree::<FilterNode>::new(self.items.clone())
    }
}

///
///
///
#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(tag = "nodeType")]
pub enum FilterNode{
    ///
    ///
    ///
    Item(FilterItem),
    ///
    ///
    ///
    Or(OrConnect),
    ///
    ///
    ///
    And(AndConnect),
}

impl LevelNode for FilterNode {

    fn get_id(&self) -> String {
        match self {
            FilterNode::Item(x) => x.get_id(),
            FilterNode::Or(x) => x.get_id(),
            FilterNode::And(x) => x.get_id(),
        }
    }

    fn get_level(&self) -> u32 {
        match self {
            FilterNode::Item(x) => x.get_level(),
            FilterNode::Or(x) => x.get_level(),
            FilterNode::And(x) => x.get_level(),
        }
    }

    fn get_text(&self) -> String {
        match self {
            FilterNode::Item(x) => x.get_text(),
            FilterNode::Or(x) => x.get_text(),
            FilterNode::And(x) => x.get_text(),
        }
    }
}
