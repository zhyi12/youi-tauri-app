use std::collections::HashMap;
use std::fmt::Formatter;
use itertools::Itertools;
use serde::Serializer;
use crate::dimension::{Dimension, Measure};
use crate::item::{Item, MeasurePart};

///
/// 合并后的rank
///
pub struct MergedRank{
    ///
    ///
    ///
    pub(crate) dimensions:Vec<Dimension>

}

///
/// 合并计量和值维度，生成MergedRank
///
impl From<&Vec<&Dimension>> for MergedRank{

    fn from(dimensions: &Vec<&Dimension>) -> Self {
        //合并计量和值维度
        let opt_measure = dimensions.iter().find(|d|match d {
            Dimension::Measure(_)=>true,
            _=>false,
        });
        //值显示维度
        let opt_measure_part = dimensions.iter().find(|d|match d {
            Dimension::Value(_)=>true,
            _=>false,
        });

        //合并指标和值显示
        let measure_items = match (opt_measure,opt_measure_part){
            (Some(measure),Some(part))=>{
                //展开的指标集合
                expand_measure_items(measure,part)
            }
            (Some(measure),None)=>{
                measure.items_clone()
            }
            (_,_)=>vec![]
        };

        //
        let mut dimensions = dimensions.iter().filter(|d|{
            match d {
                Dimension::Measure(_)|Dimension::Value(_)=>false,
                _=>true,
            }
        })
            .map(|d|(*d).clone())
            .collect::<Vec<Dimension>>();

        if !measure_items.is_empty(){
            //加入处理后的计量维度
            dimensions.push(Dimension::Measure(Measure::from(measure_items)));
        }
        Self{
            dimensions
        }
    }
}

///
/// 展开计量和值显示维度
/// 1、时间框架指标组合列 - 上年同期营业收入、上期营业收入等
/// 2、时间框架指标组合列及计算 - 上年同期营业增加值、上年同期营业增速等， expression = name_001/name*100
/// 3、含分组项的计量列（快捷处理采集中的组合指标）
///
fn expand_measure_items(measure:&Dimension,part:&Dimension)->Vec<Item>{
    match (measure,part) {
        (Dimension::Measure(m),Dimension::Value(v))=>{
            v.items.iter().map(|item|{
                match item {
                    Item::MeasurePart(x) => combine_measure_part(m,x),
                    _=>vec![]
                }
            }).flatten().collect::<Vec<Item>>()
        }
        (_,_)=>vec![]
    }
}

///
/// 组合计量
///
fn combine_measure_part(measure:&Measure,part:&MeasurePart)->Vec<Item>{
    measure.items.iter()
        .filter(|m| !m.column_name().is_empty() && match &part.measure_item_name {
            None=>true,
            Some(measure_item_name)=> m.column_name() == measure_item_name
        })
        .map(|x|{
            match x {
                Item::Measure(measure_item)=>{
                    let mut combine_measure_item = measure_item.clone();
                    // 设置原子数据项为当前组合的指标的列名
                    combine_measure_item.column.meta_item_name = Some(measure_item.column.column_name.clone());
                    // 重新设置列名
                    combine_measure_item.column.column_name = format!("comb_{}_{}",&measure_item.column.column_name,&part.id);
                    // 设置计量计算公式
                    combine_measure_item.expression =
                        Some(part.build_expression(&measure_item.measure_name()));
                    vec![Item::Measure(combine_measure_item)]
                },
                _=>vec![]
            }
        })
        .flatten()
        .collect::<Vec<Item>>()
}


impl MergedRank {

    ///
    ///
    ///
    pub fn merge(&mut self,other:&MergedRank){
        self.dimensions.iter_mut().for_each(|d|{
            let id = d.find_id();
            let items = other.dimension_items_clone(&id);
            d.merge_items(items);
        });
    }

    ///
    /// 拷贝分组项
    ///
    pub fn dimension_items_clone(&self, dim_id:&str) ->Vec<Item>{
        let opt_dim = self.dimensions.iter()
            .find(|d| &d.find_id() == dim_id);

        match opt_dim {
            None => vec![],
            Some(d) => d.items_clone()
        }
    }

    ///
    /// 替换union表的名称
    ///
    pub fn fix_union_measure(&mut self,union_mapping: &HashMap<String, String>){
        let opt_measure_dimension = self.dimensions
            .iter_mut()
            .find(|d|match d {
                Dimension::Measure(_)=>true,
                _=>false
        });

        match opt_measure_dimension {
            None => {}
            Some(measure_dimension) => {

                match measure_dimension{
                    Dimension::Measure(measure) => {
                        measure.replace_union_table_name(union_mapping);
                    }
                    _=>{}
                }
            }
        }
    }
}

///
/// 行列
///
#[derive(Clone)]
pub struct Rank<'a>{
    index:u32,
    pub(crate) dimensions:Vec<&'a Dimension>
}

//

impl<'a> Rank<'a> {

    ///
    ///
    ///
    pub fn new(index:u32,dimensions:Vec<&'a Dimension>)->Self{
        Self{
            index,
            dimensions
        }
    }
}

impl<'a> Rank<'a> {

    ///
    ///
    ///
    pub fn is_empty(&self)->bool{
        self.dimensions.is_empty()
    }
    ///
    /// 维度合并
    /// 1、如果存在多个计量维度，只保留最后面的
    /// 2、如果存在相同的分组维度，保留最后面的
    ///
    pub fn clean(&self)->Self{
        let dimensions = (0..self.dimensions.len())
            .map(|idx|((&self.dimensions[idx]).find_id(),idx))
            .into_group_map()
            .values()
            .map(|v|{
                let index = v[v.len()-1];
                self.dimensions[index]
            })
            .collect::<Vec<&Dimension>>();

        Self{
            index: self.index,
            dimensions
        }
    }

    ///
    /// 维度唯一标志，用于维度合并
    ///
    pub fn key(&self,union_mapping: &HashMap<String, String>)->String{
        self.dimensions
            .iter()
            .filter(|d|match d {
                Dimension::Value(_)=>false,
                _=>true,
            })
            .map(|d|match d {
                Dimension::Measure(m) => {
                    let table_name = m.find_table_name();
                    if union_mapping.contains_key(&table_name){
                        format!("M_{}",union_mapping.get(&table_name).unwrap())
                    }else{
                        format!("M_{}",table_name)
                    }
                }
                _=>d.find_id()
            })
            .sorted()
            .join(",")
    }
}


impl std::fmt::Debug for Rank<'_> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.serialize_str(&self.dimensions.iter().map(|d|d.find_id()).join("\n"))?;
        Ok(())
    }
}

impl std::fmt::Debug for MergedRank{

    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.serialize_str("\nRank:\n")?;
        f.serialize_str(&self.dimensions
            .iter()
            .map(|d|format!("merged rank {} ,items count {}.",d.find_id(),d.items().len()))
            .join("\n"))?;
        Ok(())
    }
}