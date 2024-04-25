use serde::{Serialize,Deserialize};
use crate::dimension::{Column, IGroup};
use crate::item::Item;
///
/// 行政区划维度
///
#[derive(Serialize, Deserialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct AreaDimension{

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

    pub(crate) items:Vec<Item>
}

///
/// &self
///
impl AreaDimension {

    pub fn id(&self)->&str{
        &self.id
    }
    ///
    ///
    ///
    pub fn parse_table_columns(&self)->Vec<(String,String)>{
        vec![(self.column.table_name.to_string(),self.column.column_name.to_string())]
    }

    pub fn items_clone(&self)->Vec<Item>{
        self.items.clone()
    }
}

///
/// &mut self
///
impl AreaDimension {
    ///
    /// 设置ITEMS
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
impl IGroup for AreaDimension {

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