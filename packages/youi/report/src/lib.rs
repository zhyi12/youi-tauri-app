
pub mod constants;
mod query_model;
mod query_dsl;
mod report;
mod report_render;
mod report_data_grid;
mod utils;
mod fmt;
pub mod dimension;
pub mod item;
mod widget;
mod model;
mod cell;
pub mod cube;
mod query;
mod dataset;
mod tree;
mod filter;
mod error;
mod param;

use std::collections::HashMap;
use std::fmt::{Debug};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::constants::DIMENSION_MEASURE;

pub use crate::filter::JoinMode;
pub use crate::model::ReportModel;
pub use crate::widget::{ReportWidget,Widget};
pub use crate::query::{QueryExecutor,ColumnSelect,SqlTranslator,QueryFunc};

pub use crate::dimension::Group;
pub use crate::item::GroupItem;
pub use crate::param::{Param,replace_param_value};

pub use youi_table::DataType;
pub use rhai::Engine;
///
/// 报表
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Report{
    ///
    /// 报表标题
    ///
    title:Option<String>,

    ///
    /// 主栏列数
    ///
    main_columns:Option<i32>,

    ///
    /// 宾栏行数
    ///
    slave_rows:Option<i32>,

    ///
    /// 是否显示宾栏代码行
    ///
    show_slave_code:Option<bool>,

    ///
    /// 表格行
    ///
    pub rows:Option<Vec<Row>>,

    ///
    /// 列信息
    ///
    col_models:Option<Vec<ColModel>>,

    ///
    /// 数据集
    ///
    dataset:Option<Vec<DataTable>>,

    ///
    /// 拐角
    ///
    corners:Option<Vec<Area>>,

    ///
    /// 合并区域
    ///
    merged_cells:Option<Vec<Area>>
}

///
/// 报表查询模型
///
pub struct  ReportQueryModel<'a>{
    ///
    /// 平铺后的数据集
    ///
    pub dataset:Vec<DataTable>,
    ///
    /// 主栏行维度集合
    ///
    pub main_ranks:Option<Vec<Rank>>,
    ///
    /// 宾栏列维度集合
    ///
    pub slave_ranks:Option<Vec<Rank>>,
    ///
    /// 立方体集合
    ///
    pub cubes:Option<Vec<Cube>>,

    ///
    /// 拐角扩展、缩进扩展
    ///
    pub extended_dimension_map:HashMap<(usize,usize),&'a Vec<Dimension>>,
}

///
///
///
#[derive(Clone,Debug)]
pub struct Rank{
    index:i32,
    dimensions:Vec<Dimension>
}
///
/// 立方体
///
#[derive(Clone,Debug)]
pub struct Cube{
    ///
    /// 立方体维度
    ///
    dimensions:Vec<Dimension>
}

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataCube{
    ///
    /// 立方体维度
    ///
    pub dimensions:Vec<Dimension>,

    ///
    ///
    ///
    pub data_map:Option<HashMap<String,DataValue>>

}

impl  DataCube {

    pub fn empty() ->DataCube{
        DataCube{
            dimensions:Vec::<Dimension>::new(),
            data_map:None
        }
    }
    ///
    ///
    ///
    pub fn new(dimensions:Vec<Dimension>,data_map:HashMap<String,DataValue>) -> DataCube {
        DataCube{
            dimensions,
            data_map:Some(data_map)
        }
    }
}

impl DataCube {

    pub fn get_dimension_items(self,id:&str)->Option<Vec<Item>>{
        let dimension = self.dimensions.iter().filter(|dim|&dim.id == id).next();
        match dimension {
            None => None,
            Some(d) => {
                match &d.items {
                    None => None,
                    Some(items) => Some(items.iter().map(|item|item.clone()).collect::<Vec<Item>>())
                }
            }
        }
    }
}

///
///
///
#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct DataValue{
    pub value:String
}

impl DataValue {
    pub fn new(value:&str)->Self{
        DataValue{
            value:value.to_string()
        }
    }
}
///
/// 查询数据集
///
pub struct QueryDataset{

    ///
    /// 数据表集合
    ///
    pub tables:Vec<DataTable>,

    ///
    /// 数据表连接集合
    ///
    pub joins:Vec<Join>
}

///
/// 表连接
///
pub struct Join{
    ///
    ///
    ///
    pub join_type:String,
    ///
    ///
    ///
    pub left:Vec<String>,
    ///
    ///
    ///
    pub right:Vec<String>,
}

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Area{
    start_row:usize,
    end_row:usize,
    start_col:usize,
    end_col:usize
}

impl Area {

    ///
    ///
    ///
    pub fn is_in_area(self:&Self,row_idx:usize,col_idx:usize)->bool{
        self.start_row<=row_idx
            && self.end_row>=row_idx
            && self.start_col<=col_idx
            && self.end_col>=col_idx
    }

    ///
    ///
    ///
    pub fn get_coordinates(self:&Self)->Vec<(usize,usize)>{
        (self.start_row..self.end_row+1).map(|row_idx|{
            (self.start_col..self.end_col+1).map(|col_idx|
                (row_idx,col_idx)
            ).collect::<Vec<(usize,usize)>>()
        }).flatten().collect::<Vec<(usize,usize)>>()
    }

}

///
///
///
#[derive(Serialize,Deserialize,Debug)]
pub struct ColModel{
    ///
    /// 列宽度
    ///
    width:Option<f64>
}

///
///
///
pub enum DataReader{
    CSV,
    SQL,
    QUERY,
}
///
/// 数据表
///
#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataTable {
    ///
    /// 表名
    ///
    name:String,

    ///
    /// csv/sql/query/...
    ///
    reader:String,
    ///
    /// 路径
    ///
    uri:String,
    ///
    /// 列集合
    ///
    columns:Vec<DataColumn>,

    ///
    /// 所在层级
    ///
    level:Option<usize>,
    ///
    /// 子表
    ///
    sub_tables:Option<Vec<DataTable>>,
    ///
    /// 从表
    ///
    join_tables:Option<Vec<DataTable>>,
}
///
/// 数据列
///
#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataColumn {
    id:Option<String>,

    ///
    /// 主键标识
    ///
    primary_key:Option<bool>,
    ///
    /// 列名
    ///
    name:Option<String>,
    ///
    /// 列中文名
    ///
    text:Option<String>,
    ///
    /// 数据类型
    ///
    data_type:Option<String>,
    ///
    /// 表达式
    ///
    expression:Option<String>
}

///
///
///
#[derive(Serialize,Deserialize,Clone)]
pub struct Row{
    height:Option<f64>,
    cells:Vec<Cell>
}

///
/// 单元格
///
#[derive(Serialize,Deserialize,Clone)]
pub struct Cell{
    ///
    ///
    ///
    pub text:Option<String>,
    ///
    ///
    ///
    dimensions:Option<Vec<Dimension>>,
    ///
    /// 交叉项
    ///
    cross_item:Option<Vec<Item>>
}

impl Cell {

    ///
    ///
    ///
    pub fn find_dimensions(&self)->Vec<Dimension>{
        match &self.dimensions {
            Some(d)=>{
                d.clone()
            }
            None=>{
                Vec::<Dimension>::new()
            }
        }
    }

    ///
    ///
    ///
    pub fn set_text(self:&mut Self,text:String){
        self.text = Some(text);
    }

}

///
/// 维度
///
#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dimension{
    ///
    /// 唯一标识
    ///
    pub id:String,
    ///
    /// 数据列名
    ///
    pub name:String,
    ///
    /// 数据表名
    ///
    table_name:Option<String>,

    ///
    /// 维度类型
    ///
    #[serde(default = "DimensionType::default")]
    dimension_type:DimensionType,
    ///
    /// 维度名称
    ///
    text:Option<String>,
    ///
    /// 维度项
    ///
    items:Option<Vec<Item>>
}

///
/// 维度类型
///
#[derive(Serialize_repr, Deserialize_repr,Clone,Debug)]
#[repr(u8)]
pub enum DimensionType{
    ///
    /// 条件分组
    ///
    Condition = 1,
    ///
    /// 代码分级分组
    ///
    Cascade = 2,
    ///
    /// 树型条件分组
    ///
    Tree = 3,
    ///
    /// 分段分组
    ///
    Segment = 4,
    ///
    ///
    ///
    Empty = 0,

}

impl DimensionType {

    pub fn default()->Self{
        DimensionType::Empty
    }
}

impl Dimension {
    ///
    ///
    ///
    pub fn new(id:&str,name:&str)->Self{
        Self{
            id: id.to_string(),
            table_name: None,
            name: name.to_string(),
            dimension_type: DimensionType::Empty,
            text: None,
            items: None
        }
    }
}

impl Dimension {

    ///
    /// 是否可展开
    ///
    pub fn is_expandable(&self)->bool{
        match &self.items {
            None => false,
            Some(items) => {
                return items.len()>1;
            }
        }
    }
    ///
    ///
    ///
    pub fn merge_items(&mut self,other_items:&Vec<Item>){
        let mut items= match &self.items {
            None => {
                Vec::<Item>::new()
            }
            Some( i) => {
                i.into_iter().map(|item|item.clone()).collect()
            }
        };

        let mut other = other_items.into_iter().map(|item|item.clone()).collect::<Vec<Item>>();
        items.append(&mut other);

        if &self.name == DIMENSION_MEASURE{
            //删除重复计量项
            items.dedup();
        }

        self.items = Some(items);
    }

    pub fn set_items(mut self,items:Vec<Item>)->Self{
        self.items = Some(items);
        self
    }
    ///
    ///
    ///
    pub fn set_id_items(&mut self,items:&Vec<String>){
        self.items = Some( items.into_iter().map(|name|Item{
            id:name.to_string(),
            name:Some(name.to_string()),
            dim_id: None,
            dim_name:None,
            aggregate: None,
            table_name: None,
            text: None,
            item_type: None,
            expression: None,
            level:None,
        }).collect::<Vec<Item>>());
    }
}
///
/// 维度项
///
#[derive(Serialize,Deserialize,Clone,PartialOrd,PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item{
    pub id:String,

    dim_id:Option<String>,

    ///
    ///
    ///
    dim_name:Option<String>,
    ///
    /// 列名
    ///
    name:Option<String>,

    ///
    /// sum/count/max/min/avg
    ///
    aggregate:Option<String>,

    ///
    /// 所在数据表名
    ///
    table_name:Option<String>,
    ///
    ///
    ///
    text:Option<String>,
    ///
    /// 项类型
    ///
    item_type:Option<String>,
    ///
    /// 表达式
    ///
    expression:Option<String>,
    ///
    ///
    ///
    level:Option<usize>
}

impl Item {
    pub fn new(id:&str)->Self{
        Item{
            id: id.to_string(),
            dim_id: None,
            dim_name:None,
            name: None,
            aggregate: None,
            table_name: None,
            text: None,
            item_type: None,
            expression: None,
            level:None
        }
    }
}

impl Item {

    ///
    ///
    ///
    pub fn set_dim_id(mut self, dim_id:String) -> Self {
        self.dim_id = Some(dim_id);
        self
    }

    ///
    ///
    ///
    pub fn set_dim_name(mut self, dim_name:String) -> Self {
        self.dim_name = Some(dim_name);
        self
    }

    ///
    ///
    ///
    pub fn set_text(mut self, text:String) -> Self {
        self.text = Some(text);
        self
    }

    ///
    ///
    ///
    pub fn set_expression(mut self, expression:String) -> Self {
        self.expression = Some(expression);
        self
    }
}

///
/// 计量项
///
#[derive(Serialize,Deserialize)]
pub struct MeasureItem{
    id:String,
    name:String,
    aggregate:Option<String>,
    expression:Option<String>
}

///
/// 数据表格
///
#[derive(Serialize,Deserialize,Debug)]
pub struct DataGrid{
    ///
    /// 行
    ///
    pub rows:Vec<Row>,

    ///
    /// 列数
    ///
    pub columns:usize,

    ///
    /// 冻结行树
    ///
    frozen_rows:usize,

    ///
    /// 冻结列数
    ///
    frozen_columns:usize,

    ///
    /// 拐角
    ///
    corners:Vec<Area>,

    ///
    /// 合并单元格
    ///
    merged_cells:Vec<Area>

}

#[cfg(test)]
mod test {

    #[test]
    fn test_report_query(){
        println!("test {}","")
    }
}