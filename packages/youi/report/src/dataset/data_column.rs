use serde::{Deserialize,Serialize};

///
/// 数据列
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataColumn{

    ///
    /// 主键标识
    ///
    primary_key:Option<bool>,

    pub(crate) name:String,

    pub(crate) text:Option<String>,

    ///
    /// 数据类型
    ///
    pub(crate) data_type:Option<String>,

    ///
    /// 列计算表达式
    ///
    expression:Option<String>,

}

impl DataColumn {
    pub fn name(&self)->&str{
        &self.name
    }

    pub fn name_clone(&self)->String{
        self.name.to_string()
    }

    pub fn is_primary_key(&self)->bool{
        match &self.primary_key {
            None => false,
            Some(primary_key) => *primary_key
        }
    }
}

