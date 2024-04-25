use std::collections::{HashMap};
use grid::Grid;
use itertools::Itertools;
use log::debug;
use serde::{Deserialize, Serialize};
use youi_table::{Area, DataType, ITable};
use crate::dimension::Dimension;
use crate::item::Item;
use crate::tree::{Tree,LevelNode};
use crate::widget::{Table,IWidget};
use crate::widget::rank::Rank;
use crate::widget::tree::{CellNode};

///
/// 统计汇总表
/// 一、布局
///     主栏
///     宾栏
///     数据区
/// 二、主宾栏元数据
///     1、计量
///         列计量
///         计算列计量（先列计算再聚合）
///         计量计算项（聚合后的计量计算）
///     2、分组维度
///         列分组
///         列映射分组（计算标签）
///         值维度（排名、占比、）
///         报告期
///         行政区划
/// 三、立方体数据查询
/// 四、立方体计算
///
/// 五、表格计算
///     行公式
///     列公式
///     单元格公式（可按区域扩展）
///
///
#[derive(Serialize, Deserialize,Debug)]
pub struct ReportWidget{

    id:String,

    ///
    /// 主栏列数
    ///
    main_columns:Option<u32>,

    ///
    /// 宾栏行数
    ///
    slave_rows:Option<u32>,

    #[serde(flatten)]
    pub(crate) table:Table

}

///
///
///
impl ReportWidget {

    ///
    ///
    ///
    pub fn new(rows:usize,cols:usize)->Self{
        Self{
            id: "".to_string(),
            main_columns: Some(2),
            slave_rows: Some(1),
            table: Table {
                grid: Grid::new(rows,cols),
                merged_cells: None,
                col_models: None,
                formulas: None
            }
        }
    }
}

impl ReportWidget {

    fn get_main_columns(&self)->u32{
        self.main_columns.unwrap_or(2)
    }

    fn get_slave_rows(&self)->u32{
        self.slave_rows.unwrap_or(1)
    }
}

impl ReportWidget {

    ///
    ///
    ///
    pub fn add_dimension(&mut self,row_index:usize,col_index:usize,dimension:Dimension){
        let opt_cell = self.table.grid.get_mut(row_index,col_index);
        match opt_cell {
            None => {}
            Some(cell) => {
                cell.add_dimension(dimension);
            }
        }
    }

    ///
    ///
    ///
    pub fn fill_dimension(&mut self,row_index:usize,col_index:usize,dim_id:&str,items:Vec<Item>){
        let opt_cell = self.table.grid.get_mut(row_index,col_index);
        match opt_cell {
            None => {}
            Some(cell) => {
                cell.fill_dimension(dim_id,items);
            }
        }
    }
    ///
    ///
    ///
    pub fn set_text(&mut self,row_index:usize,col_index:usize,text:DataType){
        let opt_cell = self.table.grid.get_mut(row_index,col_index);
        match opt_cell {
            None => {}
            Some(cell) => {
                cell.text = Some(text);
            }
        }
    }
}

impl ReportWidget {

    pub fn find_id(&self)->&str{
        &self.id
    }

    ///
    /// 分组索引
    ///
    pub fn find_idx_groups(&self)->Vec<(String,usize,usize,String)>{
        let rows = self.table.grid.rows();
        let cols = self.table.grid.cols();

        (0..rows).map(|row_index|{
            (0..cols).map(|col_index|{
                let opt_cell = self.table.grid.get(row_index,col_index);
                match opt_cell {
                    None => vec![],
                    Some(cell) => {
                        match cell.dimensions() {
                            None => vec![],
                            Some(dimensions) => {
                                dimensions.iter()
                                    .filter(|d|match d {
                                        Dimension::Group(_)|Dimension::Area(_) => true,
                                        _ => false
                                    })
                                    .map(|d|(self.id.clone(),row_index,col_index,d.find_id()))
                                    .collect::<Vec<(String,usize,usize,String)>>()
                            }
                        }
                    }
                }
            }).flatten().collect::<Vec<(String,usize,usize,String)>>()
        }).flatten().collect::<Vec<(String,usize,usize,String)>>()
    }

    ///
    /// 宾栏树
    ///
    pub fn find_slave_tree(&self,merged_map:&HashMap<(u32, u32),(u32, u32)>)->Tree<CellNode>{
        let slave_area = Area::new(0, self.get_slave_rows()-1,
                                   self.get_main_columns(), self.table.grid.cols() as u32 - 1);
        let area_texts = self.table.find_area_texts(&slave_area, merged_map);
        self.to_col_tree(&slave_area,&area_texts)
    }

    ///
    /// 主栏树
    ///
    pub fn find_main_tree(&self,_merged_map:&HashMap<(u32, u32),(u32, u32)>)->Tree<CellNode>{
        let nodes = (self.get_slave_rows()..self.table.grid.rows() as u32).map(|idx|{
            let opt_cell = self.table.grid.get(idx,0);
            match opt_cell{
                None => CellNode::new(idx,0,1,"".to_string()),
                Some(cell) => CellNode::new(idx,0,(cell.indent()+1) as u32,
                                            cell.text()),
            }
        }).collect::<Vec<CellNode>>();
        debug!("main nodes:\n{}",nodes.iter().map(|n|format!("{},{}",n.get_text(),n.get_level())).join("\n"));
        Tree::new(nodes)
    }

    ///
    /// 树转Rank集合
    ///
    pub fn tree_to_ranks(&self,tree:&Tree<CellNode>,is_row:bool)->Vec<Rank>{
        let node_paths = tree.find_node_paths();
        node_paths.iter().map(|paths|{
            let mut dimensions = paths.iter().map(|node|{
                self.table.find_cell_dimensions(node.row_index(),node.col_index())
            }).flatten().collect::<Vec<&Dimension>>();
            let index = if is_row {paths[0].row_index()}else {paths[0].col_index()};

            // debug!("paths {:?},\n dimensions {:?}",paths,dimensions);
            //含值显示的Rank处理
            self.process_value_dimension(paths[0].row_index(),paths[0].col_index(),&mut dimensions,is_row);

            Rank::new(index,dimensions)
        }).filter(|r|!r.is_empty())
            .collect::<Vec<Rank>>()
    }

    ///
    /// 宾栏值显示维度处理
    ///  当宾栏同时存在计量维度和值显示维度，且当前rank缺少计量维度，从前面区域查找计量维度，自动补充缺失的计量维度
    ///
    fn process_value_dimension<'a>(&'a self,row_index:u32,col_index:u32,dimensions:&mut Vec<&'a Dimension>,is_row:bool){
        // 计量维度在宾栏
        if !is_row{
            let opt_measure = dimensions.iter().find(|d|match d {
                Dimension::Measure(_)=>true,
                _=>false
            });

            let opt_value_dim = dimensions.iter().find(|d|match d {
                Dimension::Value(_)=>true,
                _=>false
            });

            match (opt_measure,opt_value_dim) {
                (None,Some(_))=>{
                    //在宾栏单元格查找计量维度
                    let opt_measure = self.table.find_last_measure_dimension(
                        &Area::new(0,row_index,self.main_columns.unwrap(),col_index-1)
                    );
                    //补充计量维度引用到Rank的维度引用集合中
                    match opt_measure {
                        None=>{}
                        Some(m)=> dimensions.push(m)
                    }
                }
                (_,_)=>{}
            }
        }
    }
}

///
///
///
impl IWidget for ReportWidget {
    ///
    ///
    ///
    fn find_merged_cells(&self) -> Option<&Vec<Area>> {
        self.table.merged_cells.as_ref()
    }
}

///
///
///
impl ITable for ReportWidget{

}


