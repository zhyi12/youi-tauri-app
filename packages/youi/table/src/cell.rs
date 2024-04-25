use std::fmt;
use std::fmt::{Formatter};
use youi_calamine::DataType;
use serde::{Serialize,Deserialize};
///
/// 单元格
///
#[derive(Serialize, Deserialize,Default,Clone)]
pub struct Cell{
    pub(crate) text:Option<DataType>,
    data:Option<serde_json::Value>
}

impl Cell {
    pub fn find_data(&self)->Option<&serde_json::Value>{
        (&self.data).as_ref()
    }
}

///
///
///
impl From<f64> for Cell {
    fn from(value: f64) -> Self {
        Self{
            text:Some(DataType::Float(value)),
            data:None
        }
    }
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        Self{
            text:Some(DataType::String(value.to_string())),
            data:None
        }
    }
}

///
///
///
impl From<DataType> for Cell{
    fn from(value: DataType) -> Self {
        Self{
            text:Some(value),
            data:None
        }
    }
}

impl Cell {
    ///
    ///
    ///
    pub fn new()->Self{
        Self{
            text:None,
            data:None
        }
    }

}


impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.text {
            None => f.write_str("Null")?,
            Some(text) => text.fmt(f)?
        };

        Ok(())
    }
}