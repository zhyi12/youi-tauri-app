use std::collections::HashMap;
use std::fmt::Write;
use regex::{Captures, Regex, Replacer};
use serde::{Serialize, Deserialize};
use crate::DataType;

///
/// 报表参数
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Param{
    ///
    /// 参数名
    ///
    name:String,
    ///
    /// 参数名称
    ///
    text:Option<String>,
    ///
    /// 数据类型
    ///
    data_type:Option<String>,
    ///
    /// 非空
    ///
    not_null:bool,
    ///
    /// 参数值
    ///
    value:Option<DataType>,
    ///
    /// 默认值
    ///
    default_value:Option<DataType>
}

impl Param {
    pub fn new(name:String,value:DataType)->Self{
        Self{
            name,
            text: None,
            data_type: None,
            not_null: false,
            value:Some(value),
            default_value: None
        }
    }
}

impl Param {

    ///
    ///
    ///
    pub fn value(&self)->&DataType{
        match (&self.value,&self.default_value) {
            (Some(v),_)=>v,
            (None,Some(v))=>v,
            (_,_)=>&DataType::Empty
        }
    }

    pub fn name_clone(&self)->String{
        self.name.to_string()
    }
}

///
/// 参数值替换
///
pub fn replace_param_value<'a>(script:&str,param_map:&'a HashMap<String,&'a Param>)->String{
    let regex = Regex::new("\\$\\{(?<name>[\\w_]+)\\}").unwrap();

    regex.replace_all(script,ParamReplacer{
        param_map
    }).to_string()
}

pub struct ParamReplacer<'a>{
    param_map:&'a HashMap<String,&'a Param>
}

impl<'a> Replacer for  ParamReplacer<'a>{

    ///
    ///
    ///
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let name = &caps["name"];
        let opt_param = self.param_map.get(name);
        match opt_param{
            None => {}
            Some(param) => {
                let value = param.value();
                let value_expr = match value {
                    DataType::String(x) => x.to_string(),
                    DataType::Int(x)=>x.to_string(),
                    DataType::Float(x) => x.to_string(),
                    _=>"".to_string()
                };
                dst.write_str(&value_expr).unwrap();
            }
        }
    }
}
