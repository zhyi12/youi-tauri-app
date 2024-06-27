mod filter;
mod select;
mod calculator;
mod sort;
mod join;
mod agg;
mod split;
mod reader;
mod union;

use serde::{Serialize, Deserialize};
use crate::dsl::DslNode;
use crate::step::agg::Agg;
use crate::step::calculator::Calculator;
use crate::step::filter::Filter;
use crate::step::join::Join;
use crate::step::reader::Reader;
use crate::step::select::Select;
use crate::step::sort::Sort;
use crate::step::split::Split;
use crate::step::union::Union;
#[derive(Serialize, Deserialize,Debug)]
#[serde(tag = "name")]
pub enum Step{
    ///
    /// 数据读取
    ///
    Reader(Reader),
    ///
    /// 列选择
    ///
    Select(Select),

    ///
    /// 列计算
    ///
    Calculator(Calculator),
    ///
    /// 列拆分
    ///
    Split(Split),
    ///
    /// 过滤
    ///
    Filter(Filter),
    ///
    /// 排序
    ///
    Sort(Sort),
    ///
    /// 左右连接
    ///
    Join(Join),
    ///
    /// 上下连接
    ///
    Union(Union),
    ///
    /// 汇总
    ///
    Agg(Agg),
}

impl Step {
    
    pub fn get_id(&self)->&str{
        match self {
            Step::Reader(x) => &x.info.id,
            Step::Select(x) => &x.info.id,
            Step::Calculator(x) => &x.info.id,
            Step::Split(x) => &x.info.id,
            Step::Filter(x) => &x.info.id,
            Step::Sort(x) => &x.info.id,
            Step::Join(x) => &x.info.id,
            Step::Union(x) => &x.info.id,
            Step::Agg(x) => &x.info.id,
        }
    }

    ///
    ///
    ///
    pub fn to_dsl(&self,input_step_id:Option<&str>)->crate::error::QueryResult<String>{
        match self {
            Step::Reader(x) => x.to_dsl(input_step_id),
            Step::Select(x) => x.to_dsl(input_step_id),
            Step::Calculator(x) => x.to_dsl(input_step_id),
            Step::Split(x) => x.to_dsl(input_step_id),
            Step::Filter(x) => x.to_dsl(input_step_id),
            Step::Sort(x) => x.to_dsl(input_step_id),
            Step::Join(x) => x.to_dsl(input_step_id),
            Step::Union(x) => x.to_dsl(input_step_id),
            Step::Agg(x) => x.to_dsl(input_step_id),
        }
    }
}

///
///
///
pub trait IStep{

}

#[derive(Serialize, Deserialize,Debug)]
pub struct StepInfo{

    id:String,

    text:Option<String>

}