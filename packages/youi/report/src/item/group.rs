use serde::{Deserialize, Serialize};
use crate::item::{CalculateItem,CalculateColumn,ItemComponent};

///
/// 分组维度项
///
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq,PartialOrd,Eq)]
pub struct GroupItem{
    #[serde(flatten)]
    item:ItemComponent,

    ///
    /// 项计算表达式
    ///
    expression:Option<String>,

    ///
    /// 列计算表达式
    ///
    column_expression:Option<String>
}

///
///
///
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq,PartialOrd,Eq)]
pub struct GroupSum{

}

impl GroupItem {
    pub fn new(id:&str,text:&str,level:usize)->Self{
        Self{
            item: ItemComponent {
                id: Some(id.to_string()),
                text: Some(text.to_string()),
                name: None,
                level:Some(level)
            },
            expression: None,
            column_expression: None
        }
    }
}

impl GroupItem {

    pub fn id_clone(&self)->String{
        (self.item.id.as_ref().unwrap_or(&"".to_string())).to_string()
    }

    pub fn column_expression(&self) -> &Option<String> {
        &self.column_expression
    }
}

///
/// 分组维度项计算
///
impl CalculateItem for GroupItem {

    fn get_expression(&self)->Option<&String>{
        (&self.expression).as_ref()
    }

}

///
/// 分组列映射（计算标签）
///
impl CalculateColumn for GroupItem {
    ///
    ///
    ///
    fn get_column_expression(&self)->Option<&String>{
        (&self.column_expression).as_ref()
    }
}