
use serde::{Deserialize,Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transition {
    id:String,
    pub from:String,
    pub to:String,
    expression:Option<String>
}

impl Transition {

    pub fn new(from:&str,to:&str)->Self{
        Self{
            id:format!("t_{}_{}",from,to),
            from:from.to_string(),
            to:to.to_string(),
            expression:None
        }
    }
}