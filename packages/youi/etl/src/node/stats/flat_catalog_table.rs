use itertools::Itertools;
use log::debug;
use serde::{Serialize, Deserialize};
use crate::node::{DslNode, InputtingAble};
use crate::NodeInput;

///
/// 展开的定长目录表，目录项和指标交叉形成数据存储列
/// 转换为目录列数据（交叉的列按照数据主键+目录项为行主键转换为行数据）
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlatCatalogTableTransform{

    pub(crate) id:Option<String>,
    ///
    /// 目录列名
    ///
    catalog_name:String,
    ///
    /// 主键列
    ///
    keys:Vec<String>,
    ///
    /// 输入
    ///
    inputs:Option<Vec<NodeInput>>,
    ///
    /// 目录项集合
    ///
    items:Vec<CatalogItem>,
    ///
    /// 指标集合
    ///
    indicators:Vec<Indicator>,

    ///
    /// 列起始序号
    ///
    column_num_start:usize,
    ///
    /// 列名前缀
    ///
    column_name_prefix:Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Indicator{
    pub code:String,
    pub text:String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatalogItem{
    pub code:String,
    pub text:String,
}

impl FlatCatalogTableTransform{
    ///
    ///
    ///
    pub fn set_id(&mut self,id:String){
        self.id = Some(id);
    }
}

impl InputtingAble for FlatCatalogTableTransform{
    ///
    ///
    ///
    fn set_inputs(&mut self,inputs:Vec<NodeInput>){
        self.inputs = Some(inputs)
    }
}

impl DslNode for FlatCatalogTableTransform {
    fn dsl(&self) -> Option<String> {
        debug!("{:?} {:?}",self.id,self.inputs);
        match (&self.id,&self.inputs) {
            (Some(id), Some(inputs)) => {
                if inputs.len() == 1{
                    let input = &inputs[0].id;
                    //暂时忽略map format!带来的重新分配性能损耗
                    let keys = self.keys.iter().map(|key|format!("\"{}\"",key)).join(",");
                    let indicator_codes = self.indicators
                        .iter()
                        .map(|ind|format!("\"{}\"",&ind.code)).join(",");
                    let item_codes= self.items.iter()
                        .map(|item|format!("\"{}\"",&item.code))
                        .join(",");
                    Some(format!("let df_{} = df_{}.stats_fct_transform(\"{}\",{},\"{}\",[{}],[{}],[{}])",
                        id,input,
                        self.catalog_name,self.column_num_start,self.column_name_prefix.as_ref().unwrap_or(&"".to_string()),
                        keys,item_codes,indicator_codes
                    ))
                }else{
                    None
                }
            },
            _=>None
        }
    }
}
