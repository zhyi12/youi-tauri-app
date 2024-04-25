use askama::Template;
use serde::{Deserialize,Serialize};
use crate::entity::Property;

#[derive(Template)]
#[template(path = "dao.txt")]
pub(crate) struct DaoTemplate<'a> {
    pub(crate) name: &'a str, //
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) table_name: &'a str,// 数据库表
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "tree-dao.txt")]
pub(crate) struct TreeDaoTemplate<'a> {
    pub(crate) name: &'a str, //
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate)  table_name: &'a str,// 数据库表
    // in your template
    pub(crate) properties:Vec<Property>
}
