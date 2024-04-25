use std::collections::HashMap;

use serde::{Serialize, Deserialize};
pub use crate::dataset::{tree::Node,data_union::DataUnion,data_table::DataTable,data_column::DataColumn,reader::DataReader};
use crate::filter::CubeFilter;
use crate::tree::Tree;

mod data_table;
mod data_column;
mod tree;
mod data_union;
mod reader;

///
/// 数据集
///
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dataset{

    ///
    ///
    ///
    id:Option<String>,
    ///
    /// 数据表集合
    ///
    pub(crate) tables:Vec<DataTable>,
    ///
    /// 多数据集union配置
    /// 全部数据集都只有一条路径的情况下，可以配置数据集union，
    ///
    pub(crate) data_union:Option<DataUnion>,

    ///
    /// 立方体过滤条件
    ///
    pub(crate) cube_filters:Option<Vec<CubeFilter>>

}

impl Dataset {

    pub fn tables(&self)->&Vec<DataTable>{
         self.tables.as_ref()
    }

    ///
    /// 数据集根表名集合
    ///
    pub fn dataset_names(&self)->Vec<String>{
        self.tables
            .iter()
            .map(|d|d.name_clone())
            .collect::<Vec<String>>()
    }

    ///
    ///
    ///
    pub fn find_dataset(&self,dataset_name:&str)->Option<&DataTable>{
        self.tables
            .iter()
            .find(|t|t.name() == dataset_name)
    }

    ///
    /// 查找数据集中列对应的数据表名
    ///
    pub fn find_data_table_name(&self,dataset_columns:&Vec<Vec<(&DataColumn,&str)>>,dataset_name:&str,column_name:&str)->Option<String>{
        let opt_index = (0..self.tables.len())
            .find(|index| self.tables[*index].name() == dataset_name);
        match opt_index {
            None=>None,
            Some(index)=>{
                let opt_column_idx = dataset_columns[index]
                    .iter()
                    .find(|(column,..)|column.name() == column_name);

                match opt_column_idx {
                    None=>None,
                    Some((_column,table_name))=>Some(table_name.to_string())
                }
            }
        }
    }
    ///
    /// 构建数据集树
    ///
    pub fn to_tree(&self,show_column_node:bool)->Vec<Tree<Node>>{
        self.tables
            .iter()
            .map(|table|table.to_tree(show_column_node))
            .collect::<Vec<Tree<Node>>>()
    }

    pub fn find_dataset_columns(&self)->Vec<Vec<(&DataColumn,&str)>>{
        self.tables
            .iter()
            .map(|t|t.dataset_columns())
            .collect::<Vec<Vec<(&DataColumn,&str)>>>()
    }

    ///
    /// <数据表,数据集>对照
    ///
    pub fn find_table_name_mapping(&self,table_trees:&Vec<&Tree<Node>>)->Vec<(String,String)>{
        self.tables
            .iter()
            .zip(table_trees)
            .map(|(table,tree)|table.table_name_mapping(tree))
            .flatten()
            .collect::<Vec<(String,String)>>()
    }

    ///
    /// 数据集中的合并表对照关系
    ///
    pub fn find_table_union_mapping(&self,table_trees:&Vec<&Tree<Node>>)->HashMap<String,String>{
        HashMap::from_iter(self.tables
            .iter()
            .zip(table_trees)
            .map(|(table,tree)|table.find_table_union_mapping(tree))
            .flatten()
            .collect::<Vec<(String,String)>>())
    }
}
