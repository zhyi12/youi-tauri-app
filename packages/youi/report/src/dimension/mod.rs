///!
///!  元数据
///!   1、计量、计量项
///!   2、分组维度、分组维度项、维度计算项
///!    2.1 报告期维度
///!    2.2 行政区划维度
///!    2.3 值维度
///!   3、列计算（明细列计算）
///!   4、计量计算（聚合后的计算）
///!   5、表格行、列计算项
///!

mod group;
mod measure;
mod value;
mod area;
mod period;
mod column_expression;
mod union_group;

use std::collections::HashMap;
use rayon::prelude::{IntoParallelIterator,ParallelIterator};
use serde::{Serialize, Deserialize};
pub use crate::dimension::area::AreaDimension;
pub use crate::dimension::column_expression::parse_column_names;
pub use crate::dimension::group::Group;
pub use crate::dimension::measure::Measure;
use crate::dimension::period::Period;
use crate::dimension::union_group::UnionGroup;
pub use crate::dimension::value::ValueShow;
use crate::item::Item;

///
/// 维度
///
#[derive(Serialize, Deserialize,Clone,Debug)]
#[serde(tag = "dimType")]
pub enum Dimension{
    ///
    /// 计量维度
    ///
    Measure(Measure),
    ///
    /// 分组维度
    ///
    Group(Group),

    ///
    /// 报告期维度
    ///
    Period(Period),

    ///
    /// 行政区划维度
    ///
    Area(AreaDimension),

    ///
    /// 多个数据集union生成的列维度
    ///
    Union(UnionGroup),

    ///
    /// 值显示维度，不参与汇总，可以和计量维度组合形成实际的计量输出
    ///
    Value(ValueShow)
}

impl Dimension {
    ///
    ///
    ///
    pub fn find_id(&self)->String{
        match self {
            Dimension::Measure(_) => "M".to_string(),
            Dimension::Group(x) => x.id().to_string(),
            Dimension::Area(x) => x.id().to_string(),
            Dimension::Period(_) => "P".to_string(),
            Dimension::Union(_)=>"U".to_string(),
            Dimension::Value(_)=>"V".to_string(),
        }
    }

    ///
    ///
    ///
    pub fn items(&self)->&Vec<Item>{
        match self {
            Dimension::Measure(x) => &x.items,
            Dimension::Group(x) => &x.items,
            Dimension::Area(x) => &x.items,
            Dimension::Period(x) =>  &x.items,
            Dimension::Union(x) =>  &x.items,
            Dimension::Value(x)=>&x.items
        }
    }

    ///
    ///
    ///
    pub fn items_clone(&self)->Vec<Item>{
        match self {
            Dimension::Measure(x) => x.items.clone(),
            Dimension::Group(x) => x.items.clone(),
            Dimension::Area(x) => x.items.clone(),
            Dimension::Period(x) => x.items.clone(),
            Dimension::Union(x) => x.items.clone(),
            Dimension::Value(x) => x.items.clone(),
        }
    }

    ///
    /// 分组维度列名
    ///
    pub fn find_group_column_name(&self)->String{
        match self {
            Dimension::Group(x) => x.column_name().to_string(),
            Dimension::Area(x) => x.column_name().to_string(),
            Dimension::Period(x) =>  x.column_name().to_string(),
            Dimension::Union(x)=>x.column_name().to_string(),
            _=>"".to_string()
        }
    }

    ///
    /// 列表达式
    ///
    pub fn find_column_expression(&self)->Option<String>{
        match self {
            Dimension::Group(x) => x.column_expression().cloned(),
            Dimension::Area(x) => x.column_expression().cloned(),
            Dimension::Period(x) =>  x.column_expression().cloned(),
            _=>None,
        }
    }

    ///
    /// 提取维度对应的列信息(表名、列名)
    ///
    pub fn parse_table_columns(&self)->Vec<(String,String)>{
        match self {
            Dimension::Measure(x) => x.parse_table_columns(),
            Dimension::Group(x) => x.parse_table_columns(),
            Dimension::Area(x) => x.parse_table_columns(),
            Dimension::Period(x) =>  x.parse_table_columns(),
            Dimension::Union(_x)=>vec![("t_union".to_string(),"union_col".to_string())],
            _=>vec![]
        }
    }

    ///
    /// 提取表达式中的列名
    ///
    pub fn find_column_expression_column_names(&self)->Vec<String>{
        match self {
            Dimension::Measure(x) => x.parse_ref_columns(),
            Dimension::Group(x) => x.parse_ref_columns(),
            Dimension::Area(x) => x.parse_ref_columns(),
            Dimension::Period(x) =>  x.parse_ref_columns(),
            _=>vec![],
        }
    }
}

impl Dimension {

    ///
    /// 填充项
    ///
    pub fn fill_items(&mut self,items:Vec<Item>){
        match self {
            Dimension::Measure(_x) => {},
            Dimension::Group(x) => x.fill_items(items),
            Dimension::Area(x) => x.fill_items(items),
            Dimension::Period(_x) => {},
            Dimension::Union(_x) => {},
            _=>{}
        }
    }
    ///
    /// 合并项
    ///
    pub fn merge_items(&mut self,items:Vec<Item>){
        match self {
            Dimension::Measure(x) => x.merge_items(items),
            Dimension::Group(x) => x.add_items(items),
            Dimension::Area(x) => x.add_items(items),
            Dimension::Period(x) =>  x.add_items(items),
            Dimension::Union(x) =>  x.add_items(items),
            Dimension::Value(x)=> x.add_items(items),
        }
    }
    ///
    ///
    ///
    pub fn replace_group_table_name(&mut self,table_name_mapping:&HashMap<String,String>){
        match self {
            Dimension::Group(x) => x.replace_group_table_name(table_name_mapping),
            Dimension::Period(x) => x.replace_group_table_name(table_name_mapping),
            Dimension::Area(x) => x.replace_group_table_name(table_name_mapping),
            _=>{}
        }
    }
}


///
///
///
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq,PartialOrd,Eq,Ord)]
#[serde(rename_all = "camelCase")]
pub struct Column{

    pub(crate) table_name:String,

    ///
    /// 列名
    ///
    pub(crate) column_name:String,

    ///
    /// 数据项列名
    /// 计量为派生列时使用（时间属性派生、分组项派生等）
    /// 列如：上年同期营业收入对应的营业收入数据项
    ///
    pub(crate) meta_item_name:Option<String>,

    data_type:Option<String>

}


///
/// 分组维度
///
pub trait IGroup{
    ///
    /// 返回分组维度项
    ///
    fn items(&self)->&Vec<Item>;

    ///
    /// 表名
    ///
    fn table_name(&self)->&str;

    ///
    /// 列名
    ///
    fn column_name(&self)->&str;

    ///
    /// 列表达式
    ///
    fn column_expression(&self)->Option<&String>;
    ///
    ///
    ///
    fn set_table_name(&mut self,table_name:String);

    ///
    ///
    ///
    fn add_items(&mut self,items:Vec<Item>);

    ///
    /// 替换分组的table name
    ///
    fn replace_group_table_name(&mut self,table_name_mapping:&HashMap<String,String>){
        if table_name_mapping.contains_key(self.table_name()){
            self.set_table_name(table_name_mapping.get(self.table_name()).unwrap().to_string());
        }
    }

    ///
    /// 解析分组维度项表达式中的列
    ///
    fn parse_ref_columns(&self)->Vec<String>{
        self.items()
            .into_par_iter()
            .map(|item|{
                match item {
                    Item::Group(x)=>{
                        match x.column_expression() {
                            None => vec![],
                            Some(column_expression) => parse_column_names(column_expression)
                        }
                    },
                    _=>vec![]
                }
            })
            .flatten()
            .collect::<Vec<String>>()
    }

}

