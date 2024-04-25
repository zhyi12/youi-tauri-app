use std::collections::HashMap;
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use crate::dimension::column_expression::parse_column_names;
use crate::item::{CalculateColumn, Item};


///
/// 计量维度
///
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Measure{

    ///
    ///
    ///
    pub(crate) items:Vec<Item>,

    ///
    /// 合并的表
    ///
    union_table_names:Option<Vec<String>>
}

impl From<Vec<Item>> for Measure {
    fn from(items: Vec<Item>) -> Self {
        Self{
            items,
            union_table_names:None
        }
    }
}

impl Measure {

    pub fn items(&self)->&Vec<Item>{
        &self.items
    }

    pub fn items_clone(&self)->Vec<Item>{
        self.items.clone()
    }

    ///
    /// 从计量中提取数据列名
    ///
    pub fn parse_table_columns(&self)->Vec<(String,String)>{
        self.items
            .iter()
            .map(|item|match item {
                Item::Measure(m)=>m.parse_measure_items(),
                _=>vec![]
            })
            .flatten()
            .filter(|m|m.column_expression.is_none())
            .sorted()
            .dedup()
            .map(|m|(m.table_name_clone(),m.column_name_clone()))
            .collect::<Vec<(String,String)>>()
    }

    ///
    /// 从表达式中解析列
    ///
    pub fn parse_ref_columns(&self)->Vec<String>{
        self.items
            .iter()
            .map(|item|{
                match item {
                    Item::Measure(m)=>{
                        match m.get_column_expression() {
                            None=>vec![],
                            Some(expression)=>parse_column_names(expression)
                        }
                    },
                    _=>vec![]
                }
            })
            .flatten()
            .sorted()
            .dedup()
            .collect::<Vec<String>>()
    }

    ///
    ///
    ///
    pub fn find_table_name(&self)->String{
        let opt_item = self.items.iter().find(|item|{
            match item {
                Item::Measure(m) => m.has_table_name(),
                _=> false
            }
        });

        match opt_item {
            None => "".to_string(),
            Some(item) => {
                match item {
                    Item::Measure(m) => m.table_name_clone(),
                    _=> "".to_string(),
                }
            }
        }

    }
}

impl Measure {

    ///
    ///
    ///
    pub fn merge_items(&mut self,items:Vec<Item>){
        self.items.extend(items)
    }

    pub fn set_items(&mut self,items:Vec<Item>){
        self.items = items;
    }

    ///
    /// union表拖入的计量字段处理
    ///
    pub fn replace_union_table_name(&mut self,union_mapping: &HashMap<String, String>){
        self.items.iter_mut().for_each(|item|{
            match item {
                Item::Measure(measure_item) => {
                    measure_item.replace_union_table_name(union_mapping);
                },
                _=>{}
            }
        });

        let union_table_names = self.items
            .iter()
            .map(|item|match item {
                Item::Measure(measure_item) => {
                    match &measure_item.union_table_name {
                        None => vec![],
                        Some(union_table_name) => vec![union_table_name.to_string()]
                    }
                },
                _=>vec![] })
            .flatten()
            .sorted()
            .dedup().collect();

        //排序、去除重复项目等
        let fixed_items = self.items.iter()
            .sorted()
            .dedup()
            .map(|item|item.clone())
            .collect::<Vec<Item>>();

        self.union_table_names = Some(union_table_names);
        self.items = fixed_items;
    }
}

