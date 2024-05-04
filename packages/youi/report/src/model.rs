use std::collections::HashMap;
use crate::widget::{ReportWidget, Widget};
use serde::{Serialize, Deserialize};
use crate::ColumnSelect;
use crate::constants::DF_UNION_COL_NAME;
use crate::dataset::{DataColumn, Dataset, DataTable, Node};
use crate::filter::{CubeFilter, Filter, FilterNode};
use crate::tree::Tree;

///
/// 报表模型
///
///
#[derive(Serialize,Deserialize,Debug)]
pub struct ReportModel{

    ///
    /// 数据集
    ///
    pub(crate) dataset:Option<Dataset>,

    ///
    /// 数据集全部表共有主键的全局过滤条件
    ///
    filter:Option<Filter>,

    ///
    /// 唯一标志
    ///
    id:Option<String>,
    ///
    /// 报表编码
    ///
    code:Option<String>,
    ///
    /// 报表中文名
    ///
    text:Option<String>,
    ///
    /// 报表标题
    ///
    title:Option<String>,
    ///
    /// 组件集合
    ///
    widgets:Vec<Widget>,
}

///
///
///
impl From<ReportWidget> for ReportModel{

    ///
    ///
    ///
    fn from(value: ReportWidget) -> Self {
        Self{
            id: None,
            code: None,
            text: None,
            title: None,
            dataset: None,
            filter:None,
            widgets: vec![Widget::Report(value)]
        }
    }
}

impl ReportModel {
    ///
    ///
    ///
    pub fn get_widget_mut(&mut self,id:&str)->&mut Widget{
        self.widgets.iter_mut().find(|widget|widget.find_id() == id).unwrap()
    }

    pub fn process_widget<F>(&mut self,processor:&mut F)
        where F: FnMut(&mut Widget) -> (),{
        self.widgets.iter_mut().for_each(|widget|{
            processor(widget);
        });
    }

}

impl ReportModel {
    ///
    ///
    ///
    pub fn widgets(&self)->&Vec<Widget>{
        self.widgets.as_ref()
    }

    ///
    ///
    ///
    pub fn dataset_tables(&self)->Option<&Vec<DataTable>>{
        match self.dataset.as_ref() {
            None => None,
            Some(dataset) => Some(dataset.tables())
        }
    }
    ///
    /// 数据集树
    ///
    pub fn to_dataset_tree(&self,show_column_node:bool)->Vec<Tree<Node>>{
        match &self.dataset {
            None=>vec![],
            Some(dataset)=>{
                dataset.to_tree(show_column_node)
            }
        }
    }

    ///
    /// 全局过滤
    ///
    pub fn global_filter_tree(&self)->Option<Tree<FilterNode>>{
        match &self.filter {
            None => None,
            Some(filter) => Some(filter.to_tree())
        }
    }
    ///
    ///
    ///
    pub fn find_dataset_columns(&self)->Vec<Vec<(&DataColumn,&str)>>{
        match &self.dataset {
            None=>vec![],
            Some(dataset)=>dataset.find_dataset_columns()
        }
    }

    ///
    /// 数据表所属数据集对照
    ///
    pub fn find_table_name_mapping(&self,table_trees:&Vec<&Tree<Node>>)->HashMap<String,String>{
        let mapping = match &self.dataset {
            None=>vec![],
            Some(dataset)=>{
                dataset.find_table_name_mapping(table_trees)
            }
        };
        HashMap::from_iter(mapping)
    }

    ///
    ///
    ///
    pub fn find_table_union_mapping(&self,table_trees:&Vec<&Tree<Node>>)->HashMap<String,String>{
        match &self.dataset {
            None=>HashMap::new(),
            Some(dataset)=>{
                dataset.find_table_union_mapping(table_trees)
            }
        }
    }
    ///
    /// (widget_id,row_index,col_index,group_id)
    ///
    pub fn find_idx_groups(&self)->Vec<(String,usize,usize,String)>{
        self.widgets.iter()
            .map(|w|w.find_idx_groups())
            .flatten()
            .collect::<Vec<(String,usize,usize,String)>>()
    }

    ///
    /// 查找数据集中列对应的数据表
    ///
    pub fn find_data_table_name(&self,dataset_columns:&Vec<Vec<(&DataColumn,&str)>>,dataset_name:&str,column_name:&str)->Option<String>{
        match &self.dataset {
            None=>None,
            Some(dataset)=>dataset.find_data_table_name(dataset_columns,dataset_name,column_name),
        }
    }

    ///
    /// 子数据集
    ///
    pub fn find_dataset(&self,dataset_name:&str)->Option<&DataTable>{
        match &self.dataset {
            None => None,
            Some(dataset)=>dataset.find_dataset(dataset_name)
        }
    }

    ///
    /// 是否配置数据集合并
    ///
    pub fn is_union_dataset(&self)->bool{
        match &self.dataset {
            None=>false,
            Some(dataset)=>dataset.data_union.is_some()
        }
    }

    ///
    /// 根据表名称集合查找立方体过滤条件
    ///
    pub fn find_cube_filter(&self,table_names:&Vec<String>)->Option<&CubeFilter>{
        match &self.dataset {
            None=>None,
            Some(dataset)=>{
                match &dataset.cube_filters {
                    None => None,
                    Some(cube_filters) => {
                        cube_filters
                            .iter()
                            .find(|f|f.match_table_names(table_names))
                    }
                }
            }
        }
    }

    ///
    /// 数据集合并生成的列选择
    ///
    pub fn find_dataset_union_group_select(&self)->Option<ColumnSelect>{
        match &self.dataset {
            None=>None,
            Some(dataset)=>{
                match &dataset.data_union {
                    None => None,
                    Some(data_union) => {
                        Some(ColumnSelect::new(
                            DF_UNION_COL_NAME.to_string(),
                            DF_UNION_COL_NAME.to_string(),
                            None,
                            data_union
                                .mapping
                                .iter()
                                .map(|um|{
                                    (um.text.clone(),um.expression(DF_UNION_COL_NAME))
                                }).collect::<Vec<(String,String)>>()
                        ))
                    }
                }
            }
        }
    }
}




