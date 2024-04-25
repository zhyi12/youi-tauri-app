use std::collections::HashMap;
use std::fmt::Formatter;
use pad::PadStr;
use petgraph::{Directed, Direction, Graph};
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use serde::Serializer;

pub trait TreeVisitor<T>{

    fn start(&mut self);

    fn node_start(&mut self,node:T,parent_node:Option<T>);

    fn node_end(&mut self,node:T,children:Vec<T>);

    fn end(&mut self);

}
///
/// 主栏、宾栏的元数据树结构
///
pub struct Tree<T>{
    ///
    /// 树节点
    ///
    nodes:Vec<T>,

    ///
    /// 树图，用于树路径节点获取
    ///
    graph:Graph::<Option<String>, u32, Directed>
}

impl<T:LevelNode> Tree<T>{

    ///
    ///
    ///
    pub fn new(nodes:Vec<T>)->Self{
        let graph = build_tree_graph(&nodes);
        Self{
            nodes,
            graph
        }
    }
}

impl<T:LevelNode> Tree<T>{

    ///
    /// 获取上级索引
    ///
    fn find_parent_idx(&self,idx:usize)->usize{
        let node_idx = NodeIndex::new(idx);
        let opt_parent_edge =
            self.graph.edges_directed(node_idx,Direction::Incoming).last();
        match opt_parent_edge {
            None => 0,
            Some(parent_edge) => parent_edge.source().index()
        }
    }

    ///
    ///
    ///
    pub fn nodes(&self)->&Vec<T>{
        &self.nodes
    }
    ///
    ///
    ///
    pub fn node(&self,idx:usize)->&T{
        &self.nodes[idx]
    }
    ///
    /// 子节点
    ///
    pub fn children(&self,idx:usize)->Vec<&T>{
        let node_idx = NodeIndex::new(idx+1);

        let out_edges = self.graph.edges_directed(node_idx,Direction::Outgoing);

        out_edges.map(|e|{
            let child_idx = e.target().index();
            &self.nodes[child_idx-1]
        }).collect::<Vec<&T>>()
    }

    ///
    /// idx: nodes中的位置
    ///
    ///
    pub fn parent(&self,idx:usize)->Option<&T>{
        //图序号 = 位置+1
        let parent_idx = self.find_parent_idx(idx+1);
        if parent_idx > 0 {
            Some(&self.nodes[parent_idx-1])
        }else{
            None
        }
    }

    ///
    ///
    ///
    pub fn paths(&self,idx:usize,start_idx:usize)->Vec<&T>{
        let mut nodes = vec![&self.nodes[idx]];
        //树索引位置等于节点位置+1
        let mut tree_node_idx = idx+1;
        while tree_node_idx > start_idx+1 {
            tree_node_idx = self.find_parent_idx(tree_node_idx);
            nodes.push(&self.nodes[tree_node_idx-1]);
        }
        nodes
    }
    ///
    /// 获取树全部节点的全路径
    ///
    pub fn find_node_paths(&self)->Vec<Vec<&T>>{
        let node_count = self.nodes.len();
        (0..node_count).map(|idx| {
            let tree_node_idx = idx+1;//树节点从1开始
            let mut path_idx_list = vec![tree_node_idx];
            let mut parent_idx = self.find_parent_idx(tree_node_idx);
            path_idx_list.push(parent_idx);
            while parent_idx>0 {
                parent_idx = self.find_parent_idx(parent_idx);
                path_idx_list.push(parent_idx);
            }
            //
            path_idx_list.reverse();

            path_idx_list.iter().filter(|idx|**idx>0).map(|idx|{
                &self.nodes[idx-1]
            }).collect::<Vec<&T>>()

        }).collect::<Vec<Vec<&T>>>()
    }

    ///
    /// 树遍历
    ///
    pub fn visit<'a>(&'a self,visitor:&mut dyn TreeVisitor<&'a T>){
        visit_node_idx(self,0, visitor);
    }
}

///
///
///
fn visit_node_idx<'a,T>(tree:&'a Tree<T>,idx:usize,visitor:&mut dyn TreeVisitor<&'a T>) where T:LevelNode{
    let node_idx = NodeIndex::new(idx);
    let node_edges = tree.graph.edges_directed(node_idx,Direction::Outgoing);

    if idx>0{
        let node = tree.node(idx-1);
        visitor.node_start(node,tree.parent(idx-1));
    }else{
        visitor.start();
    }

    node_edges.for_each(|e|{
        visit_node_idx(tree,e.target().index(),visitor);
    });

    if idx>0{
        let node = tree.node(idx-1);
        visitor.node_end(node,tree.children(idx-1));
    }else{
        visitor.end();
    }
}

///
/// 有序且分层级的集合构建树
///
fn build_tree_graph<T>(nodes:&Vec<T>)->Graph::<Option<String>, u32, Directed>
    where T:LevelNode
{

    let mut graph = Graph::<Option<String>, u32, Directed>::new();
    let count = nodes.len();
    let root_idx = graph.add_node(None);

    let mut level_map = HashMap::new();

    for i in 0..count {
        let cell = &nodes[i];
        let cell_idx:NodeIndex = graph.add_node(Some(cell.get_text()));
        let level = cell.get_level();

        if level == 1 {
            //顶级节点连接线
            graph.add_edge(root_idx,cell_idx,1);
        }else if level > 1{
            //非顶级节点连接线
            let parent_level = (1..level).find(|idx|{
                level_map.contains_key(&(level-idx))
            });

            match parent_level {
                None => {}
                Some(p) => {
                    let parent_node_opt = level_map.get(&(level-p));
                    match parent_node_opt {
                        None => {
                            println!("{} no parent {:?}",level,parent_level);
                        }
                        Some(parent_node) => {
                            graph.add_edge(*parent_node,cell_idx,1);
                        }
                    }
                }
            }
        }

        level_map.insert(level,cell_idx);
    }

    graph

}

pub trait LevelNode{

    fn get_id(&self)->String;
    ///
    ///
    ///
    fn get_level(&self)->u32;
    ///
    ///
    ///
    fn get_text(&self)->String;

}

impl<T: LevelNode> std::fmt::Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.nodes.len()).for_each(|idx|{
            let node = &self.nodes[idx];
            f.serialize_str(
                &format!("\n{}:{}{}-parent[{}]",idx+1,
                    "".pad_to_width_with_char((node.get_level()*2-2) as usize,' '),
                    node.get_text(),
                    self.find_parent_idx(idx+1)
            )).unwrap();
        });
        Ok(())
    }
}