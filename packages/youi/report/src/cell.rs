use youi_table::DataType;
use serde::{Serialize,Deserialize};
use crate::dimension::Dimension;
use crate::item::Item;

#[derive(Serialize, Deserialize,Debug,Clone,Default)]
pub struct Cell{

    pub(crate) text:Option<DataType>,

    indent:Option<usize>,

    ///
    /// 主宾栏单元格的行、列表达式
    ///
    expression:Option<String>,

    ///
    /// 单元格公式
    ///
    formula:Option<String>,

    ///
    /// 维度元数据集合
    ///
    dimensions:Option<Vec<Dimension>>

}

impl Cell {

    pub fn add_dimension(&mut self,dimension:Dimension){
        if self.dimensions.is_none(){
            self.dimensions = Some(vec![]);
        }

        self.dimensions.as_mut().unwrap().push(dimension);
    }

    ///
    /// 替换项
    ///
    pub fn fill_dimension(&mut self,dim_id:&str,items:Vec<Item>){
        match self.dimensions.as_mut() {
            None => {}
            Some(dimensions) => {
                let opt_dimension = dimensions.iter_mut().find(|d|&d.find_id() == dim_id);
                match opt_dimension {
                    None => {}
                    Some(dimension) => {
                        dimension.fill_items(items);
                    }
                }
            }
        }
    }
}

impl Cell {
    ///
    ///
    ///
    pub fn dimensions(&self)->Option<&Vec<Dimension>>{
        match &self.dimensions {
            None => None,
            Some(dimensions) => Some(dimensions)
        }
    }

    ///
    ///
    ///
    pub fn text(&self)->String{
        match &self.text {
            None => "".to_string(),
            Some(text) => text.as_string().unwrap_or("".to_string())
        }
    }
    ///
    /// 缩进
    ///
    pub fn indent(&self)->usize{
        match &self.indent {
            None=>0,
            Some(indent)=>*indent
        }
    }
}