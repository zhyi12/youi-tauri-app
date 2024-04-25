use std::collections::HashMap;
// use log::debug;
use crate::dataset::data_column::DataColumn;
use serde::{Deserialize,Serialize};
use crate::dataset::reader::DataReader;
use crate::dataset::tree::{JoinNode, Node, SubNode, TableNode, UnionNode};
use crate::filter::Filter;
use crate::tree::Tree;

///
/// 数据表
///
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTable{
    ///
    ///
    ///
    name:String,

    #[serde(flatten)]
    reader:DataReader,

    ///
    /// 单表过滤条件
    ///
    filter:Option<Filter>,

    ///
    ///
    ///
    text:Option<String>,

    ///
    /// 列
    ///
    pub(crate) columns:Vec<DataColumn>,

    ///
    /// 子表
    ///
    sub_tables:Option<Vec<DataTable>>,

    ///
    /// 从表
    ///
    join_tables:Option<Vec<DataTable>>,

    ///
    ///  合并表
    ///
    union_tables:Option<Vec<DataTable>>,
}

///
///
///
impl DataTable{

    pub fn name(&self)->&str{
        &self.name
    }

    pub fn reader(&self)->&DataReader{
        &self.reader
    }
    ///
    ///
    ///
    pub fn name_clone(&self)->String{
        self.name.to_string()
    }
    ///
    ///
    ///
    pub fn text_clone(&self)->String{
        (self.text.as_ref().unwrap_or(&self.name)).to_string()
    }

    ///
    /// 业务主键
    ///
    pub fn primary_keys(&self)->Vec<String>{
        self.columns
            .iter()
            .filter(|column|column.is_primary_key())
            .map(|c|c.name_clone())
            .collect::<Vec<String>>()
    }

    ///
    /// 当前数据表的合并表集合
    ///
    pub fn union_table_names(&self)->Vec<String>{
        match &self.union_tables {
            None=>vec![],
            Some(union_tables)=>{
                union_tables.iter().map(|t|t.name_clone()).collect::<Vec<String>>()
            }
        }
    }

    ///
    /// 数据表树
    ///
    pub fn to_tree(&self,show_column_node:bool)->Tree<Node>{
        let nodes = table_to_nodes(self,1,show_column_node,true);
        Tree::new(nodes)
    }

    ///
    ///
    ///
    pub fn to_join_table_map(&self)->HashMap<String,&DataTable>{
        HashMap::from_iter(to_join_table_list(self))
    }

    ///
    /// 获取数据集的全部数据表列集合
    ///
    pub fn dataset_columns(&self)->Vec<(&DataColumn,&str)>{
        dataset_columns(self)
    }

    ///
    ///
    ///
    pub fn find_table_union_mapping(&self,tree:&Tree<Node>)->Vec<(String,String)>{
        let node_paths = tree.find_node_paths();

        (0..node_paths.len()).map(|idx|{
            let paths = &node_paths[idx];
            match paths.last() {
                None => vec![],
                Some(node) => match node {
                    Node::Union(_) => {
                        let children = tree.children(idx);
                        let parent = tree.parent(idx).unwrap();

                        children.iter().map(|node|match (parent,node) {
                            (Node::Table(p),Node::Table(n))=>(n.name_clone(),p.name_clone()),
                            (_, _) => ("".to_string(),"".to_string())
                        }).collect::<Vec<(String,String)>>()
                    }
                    _ => vec![]
                }
            }
        })
            .flatten()
            .collect::<Vec<(String,String)>>()
    }

    ///
    /// 数据表和根表对照
    ///
    pub fn table_name_mapping(&self,tree:&Tree<Node>)->Vec<(String,String)>{
        let root = tree.node(0);

        match root {
            Node::Table(root_table)=>{
                tree.nodes()
                    .iter()
                    .map(|node|{
                        match node {
                            Node::Table(table)=>{
                                vec![(table.name_clone(),root_table.name_clone())]
                            },
                            _=>vec![]
                        }
                    })
                    .flatten()
                    .collect::<Vec<(String,String)>>()
            },
            _=>vec![]
        }
    }
}

///
/// 数据集
///
pub fn dataset_columns(table: &DataTable) -> Vec<(&DataColumn,&str)>{
    let mut columns = table_columns(table);

    //从表
    match &table.join_tables {
        None => {}
        Some(join_tables) => {
            columns.extend(join_tables
                .iter()
                .map(|jt|dataset_columns(jt))
                .flatten()
                .collect::<Vec<(&DataColumn,&str)>>())
        }
    }
    //子表
    match &table.sub_tables {
        None => {}
        Some(sub_tables) => {
            columns.extend(sub_tables
                .iter()
                .map(|st|dataset_columns(st))
                .flatten()
                .collect::<Vec<(&DataColumn,&str)>>())
        }
    }
    columns
}
///
///
///
fn to_join_table_list(table: &DataTable) -> Vec<(String, &DataTable)> {
    let mut table_list = vec![];
    table_list.push((table.name_clone(),table));

    match &table.join_tables {
        None => {}
        Some(join_tables) => {
            table_list.extend(join_tables
                .iter()
                .map(|jt|to_join_table_list(jt))
                .flatten());
        }
    };

    match &table.union_tables {
        None => {}
        Some(union_tables) => {
            table_list.extend(union_tables
                .iter()
                .map(|t|(t.name_clone(),t))
            );
        }
    }

    table_list
}

fn table_columns(table: &DataTable) -> Vec<(&DataColumn,&str)>{
    table.columns
        .iter()
        .map(|c|(c,table.name()))
        .collect::<Vec<(&DataColumn,&str)>>()
}

///
/// 数据表转树节点集合
///
fn table_to_nodes(table:&DataTable,level:u32,show_column_node:bool,expand:bool)->Vec<Node>{
    let node = Node::Table(TableNode::new(table.name.to_string(),table.text_clone(),level));
    //构建树
    let mut nodes = vec![];

    nodes.push(node);

    if expand {
        //子节点
        match &table.sub_tables {
            None => {}
            Some(sub_tables) => {
                nodes.extend(tables_to_nodes(
                    Node::Sub(SubNode::new(level+1)),sub_tables,level,show_column_node));
            }
        }
        //从表
        match &table.join_tables {
            None => {}
            Some(join_tables) => {
                nodes.extend(tables_to_nodes(
                    Node::Join(JoinNode::new(level+1)),join_tables,level,show_column_node));
            }
        }
    }

    //合并表
    match &table.union_tables {
        None => {}
        Some(union_tables) => {
            nodes.extend(tables_to_nodes(
                Node::Union(UnionNode::new(level+1)),union_tables,level,show_column_node));
        }
    }

    nodes
}

///
/// 子表、从表、合并表集合转树节点
///
fn tables_to_nodes(parent_node:Node,tables:&Vec<DataTable>,parent_level:u32,show_column_node:bool)->Vec<Node>{
    let mut nodes = vec![];

    let expand = match parent_node {
        Node::Table(_) => true,
        Node::Sub(_) => false,
        Node::Join(_) => false,
        Node::Union(_) => false,
        Node::Column(_) => false
    };

    if !tables.is_empty(){
        nodes.push(parent_node);
        nodes.extend(tables
            .iter()
            .map(|sub|table_to_nodes(sub,parent_level+2,show_column_node,expand))
            .flatten()
            .collect::<Vec<Node>>())
    }
    nodes
}