
use askama::Template;
use serde::{Deserialize,Serialize};

#[derive(Clone,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property{
    pub(crate) name:String,
    pub(crate) caption:String,
    pub(crate) data_type:String
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityModel {
    pub(crate) name: String, //
    pub(crate) caption:String,
    pub(crate) cname: String,// name首字母转大写
    pub(crate) table_name: String,// 数据库表
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)] // this will generate the code...
#[template(path = "entity.txt")] // using the template in this path, relative
// to the templates dir in the crate root
pub struct EntityTemplate<'a> {
    // the name of the struct can be anything
    pub(crate) name: &'a str, //
    pub(crate) cname: &'a str,// name首字母转大写
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)] // this will generate the code...
#[template(path = "sql.txt")] // using the template in this path, relative
// to the templates dir in the crate root
pub struct SqlTemplate<'a> {
    // the name of the struct can be anything
    pub(crate) name: &'a str, //
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) table_name: &'a str,// 数据库表
    // in your template
    pub(crate) properties:Vec<Property>
}