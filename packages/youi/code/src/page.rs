use askama::Template;
use serde::{Deserialize,Serialize};
use crate::entity::Property;

#[derive(Template)]
#[template(path = "page.txt")]
pub(crate) struct PageTemplate<'a> {
    pub(crate) name: &'a str,
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) module_name: &'a str,// 模块名
    pub(crate) caption: &'a str,// 中文名称
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "tree-page-layout.txt")]
pub(crate) struct TreePageTemplate<'a> {
    pub(crate) name: &'a str,
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) module_name: &'a str,// 模块名
    pub(crate) caption: &'a str,// 中文名称
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "tree-page-layout-ts.txt")]
pub(crate) struct TreePageTsTemplate<'a> {
    pub(crate) name: &'a str,
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) module_name: &'a str,// 模块名
    pub(crate) caption: &'a str,// 中文名称
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "tree-page-path-layout.txt")]
pub(crate) struct TreePagePathTemplate<'a> {
    pub(crate) name: &'a str,
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) module_name: &'a str,// 模块名
    pub(crate) caption: &'a str,// 中文名称
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "tree-page-path-layout-ts.txt")]
pub(crate) struct TreePagePathTsTemplate<'a> {
    pub(crate) name: &'a str,
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) module_name: &'a str,// 模块名
    pub(crate) caption: &'a str,// 中文名称
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "tree-page-path-dialog.txt")]
pub(crate) struct TreePagePathDialogTemplate<'a> {
    pub(crate) name: &'a str,
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) module_name: &'a str,// 模块名
    pub(crate) caption: &'a str,// 中文名称
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "tree-page-path-dialog-add.txt")]
pub(crate) struct TreePagePathDialogAddTemplate<'a> {
    pub(crate) name: &'a str,
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) module_name: &'a str,// 模块名
    pub(crate) caption: &'a str,// 中文名称
    // in your template
    pub(crate) properties:Vec<Property>
}

#[derive(Template)]
#[template(path = "form.txt")]
pub(crate) struct FormTemplate<'a> {
    pub(crate) name: &'a str,
    pub(crate) cname: &'a str,// name首字母转大写
    pub(crate) module_name: &'a str,// 模块名
    pub(crate) caption: &'a str,// 中文名称
    // in your template
    pub(crate) properties:Vec<Property>
}
