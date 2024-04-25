use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr,Deserialize_repr};
use crate::filter::Filter;

///
/// 立方体过滤
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct CubeFilter{
    ///
    /// 立方体在数据集中路径上相关的表
    ///
    table_names:Vec<String>,

    ///
    /// 过滤
    ///
    #[serde(flatten)]
    pub(crate) filter:Filter,

    ///
    /// 连接方式
    ///
    pub(crate) join_mode:JoinMode,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum JoinMode{
    ///
    /// 和Join的开始表行一致 1
    ///
    First = 1,
    ///
    /// 去除右空行 2
    ///
    Cross = 2,
    ///
    /// 保留全部 9
    ///
    Full = 9
}

impl CubeFilter {

    ///
    /// 表名称集合匹配
    ///
    pub fn match_table_names(&self,other_table_names:&Vec<String>)->bool{
        let table_names:HashSet<&String> = HashSet::from_iter(&self.table_names);
        let other_table_names:HashSet<&String> = HashSet::from_iter(other_table_names);

        let diff = table_names.difference(&other_table_names)
            .find(|_|true);
        match diff {
            None=>true,
            Some(_)=>false
        }
    }
}