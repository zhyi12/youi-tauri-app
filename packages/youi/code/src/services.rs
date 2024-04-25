use askama::Template;
use serde::{Deserialize,Serialize};
use crate::entity::Property;

#[derive(Template)]
#[template(path = "services.txt")]
pub(crate) struct ServicesTemplate<'a> {
    pub(crate) name: &'a str, //
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate)  caption:&'a str,
    pub(crate) module_name: &'a str,// 模块名
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "tree-services.txt")]
pub(crate) struct TreeServicesTemplate<'a> {
    pub(crate) name: &'a str, //
    pub(crate)  cname: &'a str,// name首字母转大写
    pub(crate) caption:&'a str,
    pub(crate)  module_name: &'a str,// 模块名
    // in your template
    pub(crate)  properties:Vec<Property>
}