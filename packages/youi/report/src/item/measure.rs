use std::cmp::Ordering;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::dimension::{Column, parse_column_names};
use crate::item::{CalculateItem,CalculateColumn, ItemComponent};

///
/// 计量项
///
#[derive(Serialize, Deserialize,Clone,Debug,Eq, Ord)]
#[serde(rename_all = "camelCase")]
pub struct MeasureItem{
    #[serde(flatten)]
    pub(crate) item:ItemComponent,

    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) column:Column,

    ///
    /// 合并表表名，用于多表合并，提取union的表名
    ///
    pub(crate) union_table_name:Option<String>,
    ///
    /// 项计算表达式
    ///
    pub(crate) expression:Option<String>,
    ///
    /// 列计算表达式
    ///
    pub(crate) column_expression:Option<String>,

    pub(crate) aggregate:String,
}

impl PartialEq<MeasureItem> for MeasureItem {
    fn eq(&self, other: &MeasureItem) -> bool {
        &self.column.table_name == &other.column.table_name
          && &self.column.column_name == &other.column.column_name
          && &self.aggregate ==  &other.aggregate
    }
}

impl PartialOrd<MeasureItem> for MeasureItem {
    fn partial_cmp(&self, other: &MeasureItem) -> Option<Ordering> {
        self.measure_name().partial_cmp(&other.measure_name())
    }
}
///
///
///
impl MeasureItem {

    ///
    /// 列名
    ///
    pub fn column_name(&self)->&str{
        &self.column.column_name
    }

    ///
    /// 计量名
    ///
    pub fn measure_name(&self)->String{
        format!("{}_{}",self.column.column_name,self.aggregate)
    }

    pub fn aggregate(&self)->&str{
        &self.aggregate
    }

    pub fn has_table_name(&self)->bool{
        !self.column.table_name.is_empty()
    }

    pub fn table_name_clone(&self)->String{
        self.column.table_name.to_string()
    }

    ///
    ///
    ///
    pub fn column_name_clone(&self)->String{
        self.column.column_name.to_string()
    }

    ///
    /// 从计量计算中提取用于汇总的列
    /// (col(\"respondent_id_count\") - col(\"respondent_id_count_001\"))/col(\"respondent_id_count_001\")*100
    /// 提取示例： respondent_id_count_001 => respondent_id_001
    ///          respondent_id_count => respondent_id
    ///
    pub fn parse_measure_items(&self)->Vec<Self>{
        match &self.expression {
            Some(expression)=>{
                parse_column_names(expression)
                    .iter()
                    .map(|name|{
                        // TODO 完善替换规则
                        let column_name = name.replace(&format!("_{}",self.aggregate),"");
                        let mut measure_item = self.clone();
                        measure_item.column.column_name = column_name;
                        //item.name 存储计算列输出的列名
                        measure_item.item.name = Some(format!("{}",measure_item.column.column_name));
                        measure_item
                    })
                    .collect::<Vec<MeasureItem>>()
            }
            None => vec![self.clone()]
        }
    }
}

impl MeasureItem {

    ///
    ///
    ///
    pub fn replace_union_table_name(&mut self,union_mapping: &HashMap<String, String>){
        if union_mapping.contains_key(&self.column.table_name){
            //存储被合并的表名
            self.union_table_name = Some(self.column.table_name.to_string());
            self.column.table_name = union_mapping.get(&self.column.table_name).unwrap().to_string();
        }
    }
}

///
/// 计量项计算
///
impl CalculateItem for MeasureItem {

    fn get_expression(&self)->Option<&String>{
        self.expression.as_ref()
    }

}

///
///
///
impl CalculateColumn for MeasureItem {
    ///
    ///
    ///
    fn get_column_expression(&self)->Option<&String>{
        (&self.column_expression).as_ref()
    }
}
//
// ///
// ///
// ///
// pub enum Aggregate{
//     ///
//     /// 求和
//     ///
//     Sum,
//     ///
//     /// 计数
//     ///
//     Count,
//     ///
//     /// 平均值
//     ///
//     Mean,
//     ///
//     /// 最大值
//     ///
//     Max,
//     ///
//     /// 最小值
//     ///
//     Min
// }
