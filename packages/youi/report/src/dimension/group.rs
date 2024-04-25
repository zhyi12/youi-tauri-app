
use serde::{Serialize, Deserialize};
use crate::DataType;
use crate::dimension::{Column, IGroup, parse_column_names};
use crate::item::Item;
///
/// 分组维度
///
#[derive(Serialize, Deserialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group{
    ///
    ///
    ///
    id:String,

    ///
    ///
    ///
    text:Option<String>,

    ///
    /// 别名
    ///
    alias:Option<String>,
    ///
    /// 列信息
    ///
    #[serde(flatten)]
    column:Column,

    ///
    ///分组维度表达式，用于通用规则的项代码映射处理，比如截取
    ///
    column_expression:Option<String>,

    ///
    /// 分段值
    ///
    sections:Option<Vec<DataType>>,

    ///
    ///
    ///
    pub(crate) items:Vec<Item>
}

impl Group {

    pub fn new(id:&str,table_name:&str,column_name:&str,items:Vec<Item>)->Self{
        Self{
            id: id.to_string(),
            text: None,
            alias: None,
            column: Column {
                table_name: table_name.to_string(),
                column_name: column_name.to_string(),
                data_type: Some("text".to_string()),
                meta_item_name:None,
            },
            column_expression:None,
            sections:None,
            items
        }
    }
}

impl Group {

    pub fn id(&self)->&str{
        &self.id
    }

}

impl Group {

    ///
    ///
    ///
    pub fn parse_table_columns(&self)->Vec<(String,String)>{
        // 分组表达式相关的列
        // 分组项表达式相关的列
        //
        match &self.column_expression {
            Some(expression)=>parse_column_names(expression).iter().map(|name|{
                (self.column.table_name.to_string(),name.clone())
            }).collect::<Vec<(String,String)>>(),
            None=>vec![(self.column.table_name.to_string(),self.column.column_name.to_string())]
        }
    }

    ///
    ///
    ///
    pub fn items_clone(&self)->Vec<Item>{
         self.items.clone()
    }
    ///
    ///
    ///
    pub fn fill_items(&mut self,items:Vec<Item>){
        if self.items.is_empty(){
            self.items = items;
        }
    }

}

///
/// 实现IGroup相关方法
///
impl IGroup for Group {

    fn items(&self)->&Vec<Item>{
        &self.items
    }

    fn table_name(&self)->&str{
        &self.column.table_name
    }

    fn column_name(&self)->&str{
        &self.column.column_name
    }

    fn column_expression(&self)->Option<&String>{
        self.column_expression.as_ref()
    }

    fn set_table_name(&mut self,table_name:String){
        self.column.table_name = table_name;
    }

    fn add_items(&mut self, items: Vec<Item>) {
        self.items.extend(items)
    }
}