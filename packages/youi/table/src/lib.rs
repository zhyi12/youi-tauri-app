// #![feature(round_char_boundary)]

pub mod xls;
pub mod formula;
mod cell;
mod column_id;
mod table;
mod style;

use serde::{Serialize, Deserialize};
pub use crate::cell::Cell;
pub use crate::formula::{Formula,replace_area_expression,func_get_regex,func_area_regex};
pub use crate::column_id::ColumnId;
pub use crate::table::{Table,TableList,Row, ColModel};
pub use youi_calamine::DataType;

#[derive(Debug,Serialize, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Area{
    pub(crate) start_row:u32,
    pub(crate)  end_row:u32,
    pub(crate) start_col:u32,
    pub(crate)  end_col:u32
}

impl Area {
    ///
    ///
    ///
    pub fn new(start_row:u32,
               end_row:u32,
               start_col:u32,
               end_col:u32)->Self{
        Self{
            start_row,
            end_row,
            start_col,
            end_col
        }
    }
}

impl Area {

    pub fn start_row(&self)->u32{
        self.start_row
    }
    pub fn end_row(&self)->u32{
        self.end_row
    }

    pub fn start_col(&self)->u32{
        self.start_col
    }
    pub fn end_col(&self)->u32{
        self.end_col
    }
}

///
///
///
pub trait ITable{

}



