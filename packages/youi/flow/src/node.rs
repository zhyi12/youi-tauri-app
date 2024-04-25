
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node{

    pub id:String,

    text:Option<String>,

    ///
    ///
    ///
    pub node_object:Option<HashMap<String,serde_json::Value>>,
}

impl Node {
    pub fn new(id:&str)->Self{
        Self{
            id:id.to_string(),
            text:None,
            node_object:None,
        }
    }
}

///
/// 抽象节点
///
pub trait AbstractNode{
    ///
    ///
    ///
    fn find_id(&self)->String;

}