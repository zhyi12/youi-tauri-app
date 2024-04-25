use std::collections::{HashMap, HashSet};
use grid::Grid;
use itertools::Itertools;
use log::debug;
use serde::{Serialize, Deserialize};
use youi_table::{Area, ColModel, Formula};
use crate::tree::{Tree};
use crate::cell::Cell;
use crate::dimension::Dimension;
use crate::item::Item;
pub use crate::widget::cross::CrossWidget;
pub use crate::widget::list::ListWidget;
pub use crate::widget::report::ReportWidget;
pub use crate::widget::table::TableWidget;
pub use crate::widget::tree::{CellNode};
pub use crate::widget::rank::{Rank,MergedRank};

mod table;
mod cross;
mod list;
mod report;
mod tree;
mod rank;

///
/// 汇总表组件
///
#[derive(Serialize, Deserialize,Debug)]
#[serde(tag = "widgetName")]
pub enum Widget{
    ///
    /// 一般表格
    ///
    Table(TableWidget),

    ///
    /// 交叉表
    ///
    Cross(CrossWidget),

    ///
    /// 统计报表
    ///
    Report(ReportWidget),

    ///
    /// 数据列表
    ///
    List(ListWidget)
}

impl Widget {

    ///
    ///
    ///
    pub fn fill_dimension(&mut self,row_index:usize,col_index:usize,dim_id:&str,items:Vec<Item>){
        match self {
            Widget::Report(x) => x.fill_dimension(row_index,col_index,dim_id,items),
            _=>{},
        }
    }
}

impl Widget {

    pub fn find_id(&self)->&str{
        match self {
            // Widget::Table(_) => {}
            // Widget::Cross(_) => {}
            // Widget::List(_) => {}
            Widget::Report(x) => x.find_id(),
            _=>"",
        }
    }

    pub fn find_idx_groups(&self)->Vec<(String,usize,usize,String)>{
        match self {
            // Widget::Table(_) => {}
            // Widget::Cross(_) => {}
            // Widget::List(_) => {}
            Widget::Report(x) => x.find_idx_groups(),
            _=>vec![],
        }
    }

}

///
/// 表格
///
#[derive(Serialize, Deserialize,Debug)]
pub struct Table{

    #[serde(flatten)]
    pub(crate) grid:Grid<Cell>,

    ///
    ///
    ///
    pub(crate) merged_cells:Option<Vec<Area>>,

    pub(crate) col_models:Option<Vec<ColModel>>,

    pub(crate) formulas:Option<Vec<Formula>>
}

impl Table {

    pub fn find_cell_dimensions(&self,row:u32,col:u32)->Vec<&Dimension>{
        let opt_cell = self.grid.get(row,col);
        match opt_cell {
            None => vec![],
            Some(cell) => {
                match cell.dimensions(){
                    None=>vec![],
                    Some(dimensions)=>{
                        dimensions.iter().map(|d|d).collect::<Vec<&Dimension>>()
                    }
                }
            }
        }
    }

    ///
    /// 查找区域内最后一个计量维度
    ///
    pub fn find_last_measure_dimension(&self,area:&Area)->Option<&Dimension>{
        let row_count = area.end_row() - area.start_row() + 1;
        let col_count = area.end_col() - area.start_col() + 1;

        let indexes = (0..col_count).map(|col|{
            let col_index =  area.start_col() + (col_count - col) - 1;
            (0..row_count).map(|row|{
                let row_index = area.start_row()+ (row_count - row) - 1;
                (row_index,col_index)
            }).collect::<Vec<(u32,u32)>>()
        }).flatten().collect::<Vec<(u32,u32)>>();

        let opt_measure_pos = indexes.iter().find(|(r,c)|{
            let opt_cell = self.grid.get(*r,*c);
            match opt_cell {
                None=>false,
                Some(cell)=>{
                    match &cell.dimensions() {
                        None=>false,
                        Some(dimensions)=>{
                            dimensions.iter().find(|d|match d {
                                Dimension::Measure(_)=>true,
                                _=>false
                            }).is_some()
                        }
                    }
                }
            }
        });

        //
        match opt_measure_pos {
            None => None,
            Some((row,col)) => {
                self.grid.get(*row,*col)
                    .unwrap()
                    .dimensions()
                    .unwrap()
                    .iter()
                    .find(|d|match d {
                        Dimension::Measure(_)=>true,
                        _=>false
                    })
            }
        }
    }

    ///
    /// 二维文本
    ///
    pub fn find_area_texts(&self,area:&Area,merged_map:&HashMap<(u32, u32),(u32, u32)>)->Vec<Vec<String>>{
        //提取空文本列
        let empty_indexes = (area.start_col()..area.end_col()+1).filter(|col|{
            let empty_count:u32 = (area.start_row()..area.end_row()+1).map(|row|{
                let text = self.find_cell_text(row,*col);
                if text.is_empty(){
                    1
                }else{
                    0
                }
            }).sum();
            empty_count == area.end_row() - area.start_row() + 1
        }).collect::<Vec<u32>>();

        debug!("empty_indexes {:?}",empty_indexes);

        (area.start_row()..area.end_row()+1).map(|row|{
            (area.start_col()..area.end_col()+1).map(|col|{
                if empty_indexes.contains(&col){
                    //空列
                    "".to_string()
                }else{
                    let mut text = self.find_ref_cell_text(row,col,merged_map);
                    //
                    if text.is_empty() && col>area.start_col(){
                        //空值单元格处理，向左遍历继续取值,返回左偏移量
                        let left_count = col - area.start_col()-1;
                        let opt_offset = (0..left_count).find(|idx|{
                            let prev_col = area.start_col() + left_count - *idx;
                            let prev_text = self.find_ref_cell_text(row,prev_col,merged_map);
                            debug!("{col} prev_col {prev_col},{left_count} {prev_text}");
                            !prev_text.is_empty()
                        });
                        match opt_offset {
                            None => {}
                            Some(offset) => {
                                let prev_col = area.start_col() + left_count - offset;
                                text = self.find_ref_cell_text(row,prev_col,merged_map);
                            }
                        }
                    }
                    //替换特殊字符
                    text.replace("/","YYY").replace("\n","")
                }
            }).collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>()
    }
    ///
    /// 获取单元格相关文本（合并单元格、拐角单元格处理）
    ///
    pub fn find_ref_cell_text(&self,row:u32,col:u32,merged_map:&HashMap<(u32, u32),(u32, u32)>)->String{
        //合并单元格处理
        if merged_map.contains_key(&(row,col)){
            let merged_start = merged_map.get(&(row,col)).unwrap();
            self.find_cell_text(merged_start.0,merged_start.1)
        }else{
            //单元格文本
            self.find_cell_text(row,col)
        }
    }

    ///
    ///
    ///
    fn find_cell_text(&self,row:u32,col:u32)->String{
        let opt_cell = self.grid.get(row,col);

        match opt_cell {
            None => "".to_string(),
            Some(cell) => match &cell.text {
                None => "".to_string(),
                Some(text) => text.as_string().unwrap_or("".to_string())
            }
        }
    }
}

///
///
///
pub trait IWidget{

    ///
    /// 获取合并区域
    ///
    fn find_merged_cells(&self) -> Option<&Vec<Area>>;

    ///
    /// 合并单元格的对照map <单元格坐标,合并区域起始坐标>
    ///
    fn find_merged_map(&self)->HashMap<(u32, u32),(u32, u32)>{
        let merged_cell_opt = self.find_merged_cells();

        let merged_idx_list:Vec<(u32, u32, u32, u32)> = match merged_cell_opt {
            None => vec![],
            Some(merged_list) => {
                merged_list.iter().map(|area|{
                    (0..area.end_row()-area.start_row()+1).map(move |x|{
                        (0..area.end_col() - area.start_col()+1).map(move |y|{
                            (x+area.start_row(),y+area.start_col(),area.start_row(),area.start_col())
                        })
                    }).flatten()
                }).flatten().collect::<Vec<(u32, u32, u32, u32)>>()
            }
        };
        //输出单元格位置的Map
        let mut merged_map:HashMap<(u32, u32),(u32, u32)> = HashMap::new();
        merged_idx_list.iter().for_each(|x|{
            merged_map.insert((x.0,x.1),(x.2,x.3));
        });

        merged_map
    }

    ///
    /// 区域转列树
    ///
    fn to_col_tree(&self,area:&Area,area_texts:&Vec<Vec<String>>)->Tree<CellNode>{
        //按列组合文本 形如 文字1/文字11,文字1/文字12,文字2/文字21 集合
        let col_texts = (area.start_col()..area.end_col()+1).map(|col|{
            let path = (area.start_row()..area.end_row()+1).map(|row|{
                format!("{}",&area_texts[(row-area.start_row()) as usize][(col - area.start_col()) as usize])
            }).filter(|t|!t.is_empty()).join("/");
            (path,col)
        }).collect::<Vec<(String,u32)>>();

        debug!("tree:\n{}",col_texts.iter().map(|(text,_)|text.to_string()).join("\n"));

        //构建路径树 (路径字符串,列索引,层级)
        let tree_paths = col_texts.iter().map(|(path,col_index)|{
            let mut parts = path.split("/").collect::<Vec<&str>>();
            let part_len = parts.len();
            (0..part_len).map(|idx|{
                if idx == part_len-1{
                    parts.dedup();
                    (format!("/{}",parts.join("/")),(part_len-1) as u32,*col_index,parts.len())
                }else{
                    let mut sub_parts = (0..idx+1).map(|pi|format!("{}",parts[pi])).collect::<Vec<String>>();
                    let row_index = sub_parts.len()-1;
                    sub_parts.dedup();
                    (format!("/{}",sub_parts.iter().filter(|p|!p.is_empty()).join("/")),row_index as u32,*col_index,sub_parts.len())
                }
            }).collect::<Vec<(String,u32,u32,usize)>>()

        }).flatten().collect::<Vec<(String,u32,u32,usize)>>();

        //删除重复
        let duplicates = tree_paths.iter().map(|(x,..)|x).duplicates().collect::<Vec<&String>>();

        let nodes = if !duplicates.is_empty(){
            let mut keys = HashSet::new();
            tree_paths.iter().filter(|(text,..)|{
                let key = text.to_string();
                if duplicates.contains(&text){
                    if keys.contains(&key){
                        return false;
                    }else {
                        keys.insert(key);
                    }
                }
                true
            }).map(|(x,row_index,col_index,level)|{
                CellNode::new(*row_index,*col_index,level.clone() as u32,x.clone())
            }).collect::<Vec<CellNode>>()
        }else {
            tree_paths.iter().map(|(x,row_index,col_index,level)|{
                CellNode::new(*row_index,*col_index,level.clone() as u32,x.clone())
            }).collect::<Vec<CellNode>>()
        };

        Tree::new(nodes)
    }
}

///
/// 含立方体的组件
///
pub trait ICubeWidget{

    ///
    ///
    ///
    fn find_cubes();
}
