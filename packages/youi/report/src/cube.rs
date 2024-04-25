use std::collections::HashMap;
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use crate::ColumnSelect;
use crate::dimension::Dimension;
use crate::item::Item;
use crate::query::DataMap;

///
/// 立方体
///
#[derive(Debug,Clone)]
pub struct Cube{

    ///
    ///
    ///
    id:String,

    ///
    /// 立方体维度集合
    ///
    dimensions:Option<Vec<Dimension>>,

    ///
    /// 脚本
    ///
    script:String,

    ///
    /// 数据
    ///
    data_map:Option<DataMap>
}

impl Cube {

    pub fn new(dimensions:Vec<Dimension>)->Self{
        Self{
            dimensions:Some(dimensions),
            id:"".to_string(),
            script:"".to_string(),
            data_map:None,
        }
    }
}

impl Cube {

    pub fn set_script(&mut self,script:String){
        self.script = script;
    }

    pub fn set_data_map(&mut self,data_map:DataMap){
        self.data_map = Some(data_map);
    }

    ///
    ///
    ///
    pub fn set_measure_items(&mut self,measure_items:Vec<Item>) {
        self.dimensions.iter_mut().for_each(|dimensions| {
            dimensions.iter_mut().for_each(|d| {
                match d {
                    Dimension::Measure(m) => {
                        m.set_items(measure_items.clone());
                    },
                    _ => {}
                };
            });
        });
    }
}

impl Cube {

    ///
    ///
    ///
    pub fn replace_group_table_name(&mut self,table_name_mapping:&HashMap<String,String>){
        match &mut self.dimensions {
            None => {}
            Some(dimensions) => {
                for idx in 0..dimensions.len(){
                    dimensions.get_mut(idx).unwrap().replace_group_table_name(table_name_mapping);
                }
            }
        }
    }
}
///
///
///
impl Cube {

    pub fn id(&self)->&str{
        &self.id
    }

    pub fn script(&self)->&str{
        &self.script
    }

    pub fn dimensions(&self) -> Option<&Vec<Dimension>> {
        self.dimensions.as_ref()
    }
    ///
    ///
    /// 列计量
    /// 计算列计量
    /// 计量计算
    ///
    pub fn find_measure_items(&self)->Option<&Vec<Item>>{
        match &self.dimensions {
            None => None,
            Some(dimensions) => {
                let opt_measure = dimensions.iter().find(|d|match d {
                    Dimension::Measure(_)=>true,
                    _=>false
                });
                match opt_measure {
                    None => None,
                    Some(measure) => Some(measure.items())
                }
            }
        }
    }

    ///
    /// 根据计量定位数据集
    ///
    pub fn find_dataset_name<'a>(&self,table_name_mapping:&'a HashMap<String,String>)->Option<&'a String>{
        let opt_measure_table = match self.find_measure_items(){
            None => None,
            Some(measure_items) => {
                let opt_measure_item = measure_items
                    .iter()
                    .find(|item|{
                        match item {
                            Item::Measure(measure_item)=> measure_item.has_table_name(),
                            _=>false,
                        }
                    });

                match opt_measure_item {
                    None => None,
                    Some(item) => match item {
                        Item::Measure(measure_item)=>Some(measure_item.table_name_clone()),
                        _=>None
                    }
                }
            }
        };

        match opt_measure_table {
            None=>None,
            Some(measure_table)=>table_name_mapping.get(&measure_table)
        }
    }

    ///
    /// 分组列名
    ///
    pub fn find_group_column_names(&self)->Vec<String>{
        match &self.dimensions {
            None => vec![],
            Some(dimensions) => dimensions
                .iter()
                .filter(|d|match d{
                    Dimension::Measure(_) => false,
                    _=>true
                }).map(|d|d.parse_table_columns()[0].1.to_string())
                .filter(|s|!s.is_empty())
                .sorted()
                .dedup()
                .collect::<Vec<String>>()
        }
    }

    ///
    /// 获取计量列选择
    ///
    pub fn find_measure_select(&self)->Vec<ColumnSelect>{
        let opt_measure_items = self.find_measure_items();
        match opt_measure_items {
            None=>vec![],
            Some(measure_items)=>{
                measure_items
                    .iter()
                    .map(|item|{
                        match item {
                            Item::Measure(measure_item)=>{
                                //计量项配置了expression时为汇总后的计算表达式
                                match measure_item.expression {
                                    None => vec![ColumnSelect::from(measure_item)],
                                    Some(_) => vec![]
                                }
                            },
                            _=>vec![]
                        }
                    })
                    .flatten()
                    .collect::<Vec<ColumnSelect>>()
            }
        }
    }

    ///
    /// 获取分组列选择
    ///
    pub fn find_group_select(&self,union_group_select:&Option<ColumnSelect>)->Vec<ColumnSelect>{
        let mut group_select = match &self.dimensions {
            None => vec![],
            Some(dimensions) =>
                dimensions
                    .into_par_iter()
                    .map(|d|match d {
                        Dimension::Group(_) | Dimension::Area(_) | Dimension::Period(_)=>vec![ColumnSelect::from(d)],
                        _=>vec![]
                    })
                    .flatten()
                    .collect::<Vec<ColumnSelect>>()
        };

        //数据集合并生成的列
        match union_group_select {
            None => {}
            Some(us) => group_select.push(us.clone())
        };

        group_select
    }

    ///
    /// 提取列表达式中的列名
    ///
    pub fn find_column_expression_column_names(&self)->Vec<String>{
        match &self.dimensions {
            None => vec![],
            Some(dimensions) => dimensions
                .iter()
                .map(|d|d.find_column_expression_column_names())
                .flatten()
                .sorted()
                .dedup()
                .collect::<Vec<String>>()
        }
    }

    ///
    /// 提取cube关联的表名
    ///
    pub fn parse_table_columns(&self)->Vec<(String,String)>{
        //计量表
        match &self.dimensions {
            None => vec![],
            Some(dimensions) => {
                dimensions
                    .iter()
                    .map(|d|d.parse_table_columns())
                    .flatten()
                    .sorted()
                    .dedup()
                    .filter(|(_,name)|!name.is_empty())
                    .collect::<Vec<(String,String)>>()
            }
        }
    }
}