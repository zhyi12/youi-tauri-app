mod measure;
mod group;
mod grid;
mod value;
mod period;
mod measure_part;

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

pub use crate::item::measure::MeasureItem;
pub use crate::item::group::{GroupItem,GroupSum};
pub use crate::item::measure_part::MeasurePart;
pub use crate::item::value::ValueItem;

#[derive(Serialize, Deserialize,Clone,Debug,Eq)]
#[serde(tag = "itemType")]
pub enum Item{
    ///
    /// 计量项
    ///
    Measure(MeasureItem),

    ///
    /// 维度项
    ///
    Group(GroupItem),
    ///
    /// 分组小计
    ///
    GroupSum(GroupSum),

    ///
    ///
    ///
    Value(ValueItem),

    ///
    /// 计量组成部分
    /// 1、时间框架及时间框架计算
    /// 2、
    ///
    MeasurePart(MeasurePart),
}

impl Item {

    ///
    /// 列名
    ///
    pub fn column_name(&self)->&str{
        match self {
            Self::Measure(x)=>&x.column.column_name,
            _=>""
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (Item::Measure(m), Item::Measure(o)) => m.cmp(&o),
            _=>Ordering::Less
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Item::Measure(m), Item::Measure(o)) => m.eq(o),
            _=>false
        }
    }
}

///
/// 维度项公共属性
///
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq,PartialOrd,Eq,Ord)]
pub struct ItemComponent{
    id:Option<String>,
    ///
    ///
    ///
    text:Option<String>,
    ///
    ///
    ///
    name:Option<String>,
    ///
    ///
    ///
    level:Option<usize>
}

///
/// 计算项
///
pub trait CalculateItem{
    ///
    ///
    ///
    fn get_expression(&self)->Option<&String>;
}

///
/// 计算列
///
pub trait CalculateColumn{
    ///
    ///
    ///
    fn get_column_expression(&self)->Option<&String>;
}
