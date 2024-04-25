use itertools::Itertools;
use crate::{Area, Cell, DataCube, DataGrid, DataValue, Dimension, Row};
use crate::constants::{DIMENSION_MEASURE, ITEM_KEY_SPLIT};

impl DataGrid {

    ///
    ///
    ///
    pub fn new(rows:Vec<Row>,frozen_rows:usize,frozen_columns:usize)->Self{
        Self{
            rows,
            frozen_rows,
            frozen_columns,
            columns:0,
            corners:vec![],
            merged_cells:vec![]
        }
    }

}

///
/// setter
///
impl DataGrid {
    ///
    /// 设置corners
    ///
    pub fn set_corners(mut self,corners:Vec<Area>)->Self{
        self.corners = corners;
        self
    }

}

///
/// setter
///
impl DataGrid {

    ///
    ///
    ///
    pub fn find_cell_dimensions(self:&Self,row_idx:usize,col_idx:usize) ->Option<(Vec<&Dimension>,Vec<&Dimension>)>{
        let opt_cell = self.find_cell(row_idx, col_idx);

        match opt_cell {
            None => {}
            Some(cell) => {
                match &cell.dimensions {
                    None => {}
                    Some(dimensions) => {
                        let expandable_dimensions =dimensions.iter()
                            .filter(|dim| dim.items.is_some() && !dim.items.as_ref().unwrap().len()>1)
                            .collect::<Vec<&Dimension>>();

                        let one_item_dimensions =dimensions.iter()
                            .filter(|dim| dim.items.is_some() && !dim.items.as_ref().unwrap().len()==1)
                            .collect::<Vec<&Dimension>>();

                        if !expandable_dimensions.is_empty(){
                            return Some((expandable_dimensions,one_item_dimensions));
                        }
                    }
                }
            }
        };
        None
    }

    ///
    /// 根据行列定位查找单元格
    ///
    pub fn find_cell(self:&Self, row_index:usize, column_index:usize) -> Option<&Cell> {
        if self.rows.len() > row_index {
            let row = &self.rows[row_index];
            if row.cells.len() > column_index{
                return Some(&row.cells[column_index]);
            }
        }
        None
    }

    fn find_cell_cross_item(self:&Self, row_idx:usize, col_idx:usize)->Option<Vec<(String,String)>>{
        let opt_cell = self.find_cell(row_idx,col_idx);

        match opt_cell {
            None => {}
            Some(cell) => {
                match &cell.dimensions {
                    None => {}
                    Some(dimensions) => {
                        return Some(dimensions.iter().map(|dimension|{
                            let item = dimension.items.as_ref().unwrap().first().unwrap();
                            (dimension.name.to_string(),item.id.to_string())
                        }).collect::<Vec<(String,String)>>())
                    }
                }
            }
        }

        None
    }
}

impl DataGrid {

    ///
    /// 行集合替换
    ///
    pub fn replace_rows(self:&mut Self,index:usize,replace_rows:Vec<Row>){
        //
        self.rows.splice(index..index+1,replace_rows);
    }

    ///
    /// 单元格集合替换
    ///
    pub fn replace_cols(self:&mut Self,row_idx:usize,index:usize,replace_cells:Vec<Cell>){
        self.rows[row_idx].cells.splice(index..index+1,replace_cells);
    }

    ///
    /// 设置单元格显示文本
    ///
    pub fn replace_cell_text(self:&mut Self,row_idx:usize,col_idx:usize,text:String){
        //
        self.rows[row_idx].cells[col_idx].text = Some(text);
    }

    ///
    /// 获取列交叉项集合
    ///
    fn find_col_cross_items(&self)->Vec<Vec<(String,String)>>{
        let mut col_cross_items:Vec<Vec<(String,String)>> = vec![];

        for col_idx in self.frozen_columns..self.columns{
            let mut cross_items:Vec<(String,String)> = vec![];
            for row_idx in 0..self.frozen_rows{
                let opt_cell_cross_item = self.find_cell_cross_item(row_idx,col_idx);
                match opt_cell_cross_item {
                    None => {}
                    Some(mut cell_cross_item) => {
                        cross_items.append(&mut cell_cross_item);
                    }
                }
            }
            col_cross_items.push(cross_items);
        }

        col_cross_items
    }
    ///
    /// 渲染数据区域
    ///
    pub fn render_by_cubes(self:&mut Self,data_cubes:&Vec<DataCube>){

        let col_cross_items = self.find_col_cross_items();

        for row_idx in self.frozen_rows..self.rows.len(){
            let opt_row_cross_item = self.find_cell_cross_item(row_idx,0);
            let row_cross_item:Vec<(String,String)> = opt_row_cross_item.unwrap_or(Vec::<(String,String)>::new());

            for col_idx in self.frozen_columns..self.columns{
                let opt_cell_key = build_cell_cross_key(&row_cross_item,&col_cross_items[col_idx - self.frozen_columns]);
                match  opt_cell_key{
                    None => {}
                    Some(cell_key) => {
                        let opt_data_value = find_cell_value(&cell_key,data_cubes);
                        match opt_data_value {
                            None => {}
                            Some(data_value) => {
                                self.replace_cell_text(row_idx,col_idx,data_value.value.clone());
                            }
                        }
                    }
                }
            }
        }
    }

}

///
/// 构建数据单元格key
///
fn build_cell_cross_key(row_cross_item:&Vec<(String,String)>,col_cross_item:&Vec<(String,String)>)->Option<String>{
    let mut keys =
        row_cross_item.iter()
            .map(|(dim_id,id)|format!("{}{}{}",dim_id,ITEM_KEY_SPLIT,id))
            .collect::<Vec<String>>();

    keys.append(&mut col_cross_item.iter()
        .map(|(dim_id,id)|format!("{}{}{}",dim_id,ITEM_KEY_SPLIT,id))
        .collect::<Vec<String>>());

    keys.sort();

    let opt_measure_key =
        keys.iter().filter(|key|key.starts_with(&format!("{}{}",DIMENSION_MEASURE,ITEM_KEY_SPLIT))).next();

    match opt_measure_key {
        None => {}
        Some(measure_key) => {
            let groups_key = keys.iter().filter(|key|!key.starts_with(&format!("{}{}",DIMENSION_MEASURE,ITEM_KEY_SPLIT)))
                .join(ITEM_KEY_SPLIT);
            return Some(format!("{}{}{}",groups_key,ITEM_KEY_SPLIT,measure_key));
        }
    }
    None
}
///
/// 根据key从数据立方体中获取数据值
///
fn find_cell_value<'a>(cell_key:&'a str, data_cubes:&'a Vec<DataCube>) -> Option<&'a DataValue>  {
    for i in 0..data_cubes.len(){
        match &data_cubes[i].data_map{
            None => {}
            Some(data_map) => {

                if data_map.contains_key(cell_key){
                    return data_map.get(cell_key);
                }

            }
        }
    }
    None
}

impl DataGrid {

    ///
    ///
    ///
    pub fn to_html(&self)->String{
        let mut html = String::new();
        html.push_str("<meta content=\"text/html; charset=utf-8\" http-equiv=\"Content-Type\">");
        html.push_str("<table style=\"border-spacing:0;\" border=\"1\">");
        html.push_str(self.rows.iter().map(|row|{
            format!("<tr>{}</tr>",row.cells.iter()
                .map(|cell|format!("<td>{}<div>{}</div></td>",
                                   cell.text.as_ref().unwrap_or(&"".to_string()),build_meta_html(cell))
                ).join(""))
        }).join("\n").as_str());
        html.push_str("</table>");
        html
    }
}

///
///
///
fn build_meta_html(cell:&Cell)->String{
    let mut html = String::new();

    match  &cell.dimensions{
        None => {}
        Some(dimensions) => {
            html.push_str(dimensions.iter().map(|d|format!("<div>{}.{:?}</div>",d.id,
                                                           d.items.as_ref().unwrap().first().unwrap())).join("\n").as_str())
        }
    }

    html
}