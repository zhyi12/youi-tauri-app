
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Order{

    ///
    ///
    ///
    pub(crate) property:String,

    ///
    /// 逆序
    ///
    pub(crate) descending:Option<bool>,

}