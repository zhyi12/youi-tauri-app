use std::collections::HashMap;
use itertools::Itertools;
use log::debug;
use crate::{ReportWidget};
use crate::cube::Cube;
use crate::tree::Tree;
use crate::widget::{CellNode, IWidget, Rank,MergedRank};

///
///
///
pub enum WidgetExecutor<'a>{
    ///
    ///
    ///
    Report(ReportWidgetExecutor<'a>),

    ///
    ///
    ///
    Empty

}

impl<'a> WidgetExecutor<'a> {

    ///
    ///
    ///
    pub fn data_cube_list(&self,union_mapping: &HashMap<String, String>)->Vec<Cube>{
        match self {
            WidgetExecutor::Report(x) => x.data_cube_list(union_mapping),
            WidgetExecutor::Empty => vec![]
        }
    }
}

///
///
///
pub struct ReportWidgetExecutor<'a>{

    pub(crate) widget:&'a ReportWidget,
    ///
    /// 合并单元格映射
    ///
    pub(crate) merged_map:HashMap<(u32, u32),(u32, u32)>,

    ///
    /// 主栏树
    ///
    pub(crate) main_tree:Tree<CellNode>,

    ///
    /// 宾栏树
    ///
    pub(crate) slave_tree:Tree<CellNode>,

    ///
    /// 主栏块
    ///
    pub(crate) row_ranks:Vec<Rank<'a>>,

    ///
    /// 宾栏块
    ///
    pub(crate) col_ranks:Vec<Rank<'a>>,
}

///
///
///
impl<'a> From<&'a ReportWidget> for ReportWidgetExecutor<'a>{

    fn from(widget: &'a ReportWidget) -> Self {
        //生成合并单元格映射
        let merged_map = widget.find_merged_map();
        //生成主栏树
        let main_tree = widget.find_main_tree(&merged_map);
        //生成宾栏树
        let slave_tree = widget.find_slave_tree(&merged_map);
        //提取主栏块集合
        let row_ranks = widget.tree_to_ranks(&main_tree,true);
        //提取宾栏块集合
        let col_ranks = widget.tree_to_ranks(&slave_tree,false);

        Self{
            widget,
            merged_map,
            main_tree,
            slave_tree,
            row_ranks,
            col_ranks
        }
    }
}

impl<'a> ReportWidgetExecutor<'a> {

    ///
    /// 主宾栏交叉生成立方体集合
    ///
    pub fn data_cube_list(&self,union_mapping: &HashMap<String, String>)->Vec<Cube>{

        let merged_row_ranks = merge_ranks(&self.row_ranks,union_mapping);
        let merged_col_ranks = merge_ranks(&self.col_ranks,union_mapping);

        debug!("self.row_ranks:{:?}",&self.row_ranks);
        debug!("merged_row_ranks:{:?}",merged_row_ranks);
        debug!("merged_col_ranks: {}个Rank，{:?}",merged_col_ranks.len(),merged_col_ranks);

        merged_row_ranks
            .iter()
            .cartesian_product(&merged_col_ranks)
            .map(|(r1,r2)|{
                let dimensions = vec![r1.dimensions.clone(),r2.dimensions.clone()].concat();
                Cube::new(dimensions)
            }).collect::<Vec<Cube>>()
    }
}

///
/// rank 合并
/// 1、清理Rank内相同ID的维度
/// 2、横向分组相同Rank key 的rank
///
fn merge_ranks(ranks:&Vec<Rank>, union_mapping: &HashMap<String, String>)->Vec<MergedRank>{
    debug!("ranks {:?}",ranks);
    ranks
        .iter()
        .filter(|r|!r.is_empty())
        .map(|r|r.clean())
        .map(|rank|(rank.key(union_mapping),rank.clone()))
        .into_group_map()
        .iter()
        .map(|(_key,group_ranks)|merge_group_ranks(group_ranks,union_mapping))
        .collect::<Vec<MergedRank>>()
}

///
/// 合并同组Rank的分组项
///
fn merge_group_ranks(ranks:&Vec<Rank>, union_mapping: &HashMap<String, String>)->MergedRank{
    let mut merged_rank = MergedRank::from(&ranks[0].dimensions);
    if ranks.len()>1{
        //合并
        (1..ranks.len()).for_each(|idx|{
            merged_rank.merge(&MergedRank::from(&ranks[idx].dimensions));
        });
    }

    //处理合并表的计量
    merged_rank.fix_union_measure(union_mapping);

    merged_rank
}