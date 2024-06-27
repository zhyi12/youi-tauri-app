use serde::{Serialize, Deserialize};
use crate::dsl::DslNode;
use crate::step::StepInfo;

///
/// 排序
///
#[derive(Serialize, Deserialize,Debug)]
pub struct Split{
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,

    ///
    /// 分割符号
    ///
    pub(crate) delimiter:Option<String>,

}

///
///
///
impl DslNode for Split {

    ///
    ///
    ///
    fn to_dsl(&self,_input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        Ok(format!("let df_{} = {}","",""))
    }
}