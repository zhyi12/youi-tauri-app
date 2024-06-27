mod connect;
mod item;
mod operator;
mod dsl;

use serde::{Serialize, Deserialize};
use youi_tree::{Tree,LevelNode};

pub use item::FilterItem;
pub use operator::FilterOperator;
pub use connect::{AndConnect,OrConnect};
pub use dsl::DslFilterTreeVisitor;
///
/// 过滤条件
///
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Filter{
    ///
    ///
    ///
    pub(crate) items:Vec<FilterNode>
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