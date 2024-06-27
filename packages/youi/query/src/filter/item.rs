
use serde::{Serialize, Deserialize};
use crate::DataType;

///
/// 过滤条件
///
#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct FilterItem{
    ///
    ///
    ///
    id:String,
    ///
    /// 所属数据表
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    data_table_name:Option<String>,
    ///
    /// 属性
    ///
    pub(crate) property:String,

    ///
    ///
    ///
    text:Option<String>,
    ///
    /// 比较符
    /// eq|gt|gte|lt|lte|start_with|end_with|like|in
    ///
    pub(crate) operator:String,
    ///
    /// 值
    ///
    pub(crate) value:Vec<DataType>,

    ///
    /// 数据类型
    ///
    #[serde(default = "default_data_type")]
    pub(crate) data_type:String,

    level:u32,
}

fn default_data_type()->String{
    "text".to_string()
}

///
///
///
impl FilterItem{

    pub fn property(&self)->&str{
        &self.property
    }

    pub fn operator(&self)->&str{
        &self.operator
    }

    pub fn value(&self)->&Vec<DataType>{
        self.value.as_ref()
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_text(&self) -> String {
        self.text.as_ref().unwrap_or(&"".to_string()).to_string()
    }

    ///
    ///
    ///
    pub fn get_value_str(&self)->String{
        let value = if self.value.is_empty() {&DataType::String("".to_string())} else {&self.value[0]};
        self.value_utf8(value)
    }
    ///
    ///
    ///
    pub fn value_utf8(&self,value:&DataType)->String{
        match value {
            DataType::Int(x) => x.to_string(),
            DataType::Float(f) => f.to_string(),
            DataType::String(x) => x.to_string(),
            _ => "".to_string()
        }
    }

    ///
    ///
    ///
    pub fn get_quote_value(&self,quote_str:&str)->String{
        self.get_wrap_value(quote_str,"","")
    }

    ///
    ///
    ///
    pub fn get_quote_values(&self,quote_str:&str)->Vec<String>{
        let quote = match self.data_type.as_str() {
            "text"=>quote_str,
            _=>""
        };
        self.value.iter().map(|v|
            format!("{}{}{0}",quote,self.value_utf8(v))
        ).collect::<Vec<String>>()
    }

    pub fn get_wrap_value(&self,quote_str:&str,prefix:&str,postfix:&str)->String{
        let quote = match self.data_type.as_str() {
            "text"=>quote_str,
            _=>""
        };
        let value = self.get_value_str();

        format!("{}{}{}{}{0}",quote,prefix,value,postfix)
    }


}
