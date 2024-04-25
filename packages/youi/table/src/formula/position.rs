use hashbrown::HashMap;
use itertools::Itertools;
use regex::{Regex, Replacer, Captures};
use crate::column_id::ColumnId;

///
/// 区域公式正则匹配
///
pub fn area_regex()->Regex{
    Regex::new("(\'(?<name>\\S+)\'!)??(?<col_start>[A-Z]+)(?<row_start>[0-9]+):(?<col_end>[A-Z]+)(?<row_end>[0-9]+)").unwrap()
}

///
/// 单元格公式正则匹配
///
pub fn cell_regex()->Regex{
    Regex::new("(\'(?<name>\\S+)\'!)(?<col_index>[A-Z]+)(?<row_index>[0-9]+)").unwrap()
}
///
///
///
pub fn short_cell_regex()->Regex{
    Regex::new("(?<col_index>[A-Z]+)(?<row_index>[0-9]+)$").unwrap()
}

///
///
///
pub fn func_area_regex()->Regex{
    Regex::new("AREA\\((?<row1>[0-9]+),(?<col1>[0-9]+),(?<row2>[0-9]+),(?<col2>[0-9]+)\\)").unwrap()
}

///
///
///
pub fn func_get_regex()->Regex{
    Regex::new("GET\\((?<row>[0-9]+),(?<col>[0-9]+)\\)").unwrap()
}



///
///
///
pub struct CellPositionSwapper<'a>{
    pub(crate) main_grid_name:String,
    pub(crate) table_names:&'a HashMap<String,String>
}

impl Replacer for  CellPositionSwapper<'_>{
    ///
    ///
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let opt_name = caps.name("name");
        dst.push_str("T_");
        match opt_name {
            None => {
                dst.push_str(self.table_names.get(&self.main_grid_name).unwrap());
            },
            Some(name)=> {
                dst.push_str(self.table_names.get(name.as_str()).unwrap());
            },
        };
        dst.push_str(".");
        let col_index = ColumnId::from(&caps["col_index"]).index().to_string();

        dst.push_str("GET(");
        dst.push_str(&caps["row_index"]);
        dst.push_str(",");
        dst.push_str(&col_index);
        dst.push_str(")");
    }
}

///
/// 区域位置处理
///
pub struct AreaExpressionSwapper<'a>{
    pub(crate) main_grid_name:String,
    pub(crate) table_names:&'a HashMap<String,String>
}

impl Replacer for AreaExpressionSwapper<'_>{

    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let opt_name = caps.name("name");
        dst.push_str("T_");
        match opt_name {
            None => {
                dst.push_str(self.table_names.get(&self.main_grid_name).unwrap());
            },
            Some(name)=> {
                dst.push_str(self.table_names.get(name.as_str()).unwrap());
            },
        };
        dst.push_str(",");

        let col_start =  ColumnId::from(&caps["col_start"]).index().to_string();
        let col_end = ColumnId::from(&caps["col_end"]).index().to_string();
        let pos = vec![caps["row_start"].to_string(),caps["row_end"].to_string(),
                       col_start, col_end];
        dst.push_str("AREA(");
        dst.push_str(&pos.iter().join(","));
        dst.push_str(")");
    }
}

///
///
///
pub struct FuncGetRowExpandReplacer{
    pub(crate) offset:u32
}

impl Replacer for FuncGetRowExpandReplacer{
    ///
    ///
    ///
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let index = (&caps["row"]).parse::<u32>().unwrap();
        dst.push_str("GET(");
        dst.push_str(&(index+self.offset).to_string());
        dst.push_str(",");
        dst.push_str(&caps["col"]);
        dst.push_str(")");
    }
}

pub struct FuncGetColExpandReplacer{
    pub(crate) offset:u32
}

impl Replacer for FuncGetColExpandReplacer{
    ///
    ///
    ///
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let index = (&caps["col"]).parse::<u32>().unwrap();
        dst.push_str("GET(");
        dst.push_str(&caps["row"]);
        dst.push_str(",");
        dst.push_str(&(index+self.offset).to_string());
        dst.push_str(")");
    }
}

pub struct FuncAreaRowExpandReplacer{
    pub(crate) offset:u32
}

impl Replacer for FuncAreaRowExpandReplacer{
    ///
    ///
    ///
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let row1 = (&caps["row1"]).parse::<u32>().unwrap();
        let row2 = (&caps["row2"]).parse::<u32>().unwrap();
        dst.push_str("AREA(");
        dst.push_str(&(row1+self.offset).to_string());
        dst.push_str(",");
        dst.push_str(&caps["col1"]);
        dst.push_str(",");
        dst.push_str(&(row2+self.offset).to_string());
        dst.push_str(",");
        dst.push_str(&caps["col2"]);
        dst.push_str(")");
    }
}

pub struct FuncAreaColExpandReplacer{
    pub(crate) offset:u32
}

impl Replacer for FuncAreaColExpandReplacer{
    ///
    ///
    ///
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let col1 = (&caps["col1"]).parse::<u32>().unwrap();
        let col2 = (&caps["col2"]).parse::<u32>().unwrap();
        dst.push_str("AREA(");
        dst.push_str(&caps["row1"]);
        dst.push_str(",");
        dst.push_str(&(col1+self.offset).to_string());
        dst.push_str(",");
        dst.push_str(&caps["row2"]);
        dst.push_str(",");
        dst.push_str(&(col2+self.offset).to_string());
        dst.push_str(")");
    }
}