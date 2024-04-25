
pub mod node;
pub mod transition;
mod dag;

use itertools::Itertools;
use petgraph::prelude::NodeIndex;
use petgraph::visit::{Topo, Walker};
use serde::{Deserialize, Serialize};
pub use crate::dag::Dag;
use crate::node::{Node};
use crate::transition::Transition;

///
///
///
pub trait Visitor{

    ///
    /// 遍历节点
    ///
    fn visit_node(&mut self,node:&Node,parents:Vec<(&Node,&Transition)>,node_index:usize);

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flow {

    ///
    /// 流程节点
    ///
    nodes:Vec<Node>,

    ///
    /// 连接线
    ///
    transitions:Option<Vec<Transition>>,

    ///
    /// 有向无环图
    ///
    #[serde(skip_serializing,skip_deserializing)]
    dag:Option<Dag::<Node, Transition>>
}


impl  Flow {

    ///
    ///
    ///
    pub fn new(nodes:Vec<Node>,transitions:Vec<Transition>)->Self{

        let mut flow = Self{
            nodes,
            transitions:Some(transitions),
            dag:None
        };

        flow.fill_dag();

        flow
    }

}

impl  Flow {

    ///
    /// 填充有向无环图
    ///
    pub fn fill_dag(&mut self){

        let mut dag = Dag::<Node, Transition>::new();

        self.nodes.iter().for_each(|node|{
            dag.add_node(node.clone());
        });

        match  &self.transitions{
            None => {}
            Some(transitions) => {
                transitions.iter().for_each(|transition|{
                    let from_index = find_index(&self.nodes,transition.from.as_str());
                    let to_index = find_index(&self.nodes,transition.to.as_str());

                    dag.add_edge(NodeIndex::new(from_index),NodeIndex::new(to_index),transition.clone()).unwrap();
                });
            }
        }

        self.dag = Some(dag);
    }

}

impl  Flow {

    ///
    /// 子流程
    ///
    pub fn sub_flow(&self,node_id:&str)->Option<Self>{

        let index = find_index(&self.nodes,node_id);
        match &self.dag {
            None => None,
            Some(dag) => {
                let paths = find_graphic_path(dag,index);

                let mut node_indexes = vec![vec![0,index],paths.0].concat();
                let mut edge_indexes = paths.1;

                node_indexes.sort();
                edge_indexes.sort();

                let sub_nodes = node_indexes.iter().dedup().map(|idx|self.nodes[*idx].clone()).collect::<Vec<Node>>();
                let sub_transitions = edge_indexes.iter().dedup().map(|idx|
                    {
                        (self.transitions.as_ref().unwrap()[*idx]).clone()

                    }).collect::<Vec<Transition>>();

                let mut sub_flow = Self::new(sub_nodes,sub_transitions);

                sub_flow.fill_dag();

                Some(sub_flow)
            }
        }
    }


    ///
    /// 节点遍历
    ///
    pub fn visit(&self,visitor:&mut dyn Visitor)->Vec<&Node>{
        let mut nodes = vec![];
        match (&self.dag,&self.transitions) {
            (Some(dag),Some(transitions)) => {
                let graph = dag.graph();
                let mut topo = Topo::new(graph);
                let mut next = topo.next(graph);
                while next.is_some(){
                    let node_index = next.unwrap();
                    let node = &dag[node_index];

                    //父节点
                    let parents = dag.parents(node_index);

                    let parent_nodes = parents.iter(dag).map(|(edge_index,node_index)|{
                        (&self.nodes[node_index.index()],&transitions[edge_index.index()])
                    }).collect::<Vec<(&Node,&Transition)>>();

                    nodes.push(node);

                    visitor.visit_node(node,parent_nodes,node_index.index());

                    //下一个节点
                    next = topo.next(graph);
                }
            },
            _ => {}

        };

        nodes
    }
}

///
///
///
fn find_index(nodes:&Vec<Node>,id:&str)->usize{
    let node_count =  nodes.len();
    (0..node_count).map(|idx|{
        if nodes[idx].id.as_str() == id {
            idx
        }else{
            0
        }
    }).sum::<usize>()
}

///
///
///
fn find_graphic_path(dag:&Dag<Node, Transition>,node_index:usize)->(Vec<usize>,Vec<usize>){

    let parents = dag.parents(NodeIndex::new(node_index));
    let mut node_indexes:Vec<usize> = vec![];
    let mut edge_indexes:Vec<usize> = vec![];

    parents.iter(dag).for_each(|(edge_index,node_index)|{
        let parent_index = node_index.index();
        edge_indexes.push(edge_index.index());
        if parent_index > 0 {
            node_indexes.push(parent_index);
            //
            let parent_path = find_graphic_path(dag,parent_index);
            node_indexes = vec![node_indexes.clone(),parent_path.0].concat();
            edge_indexes = vec![edge_indexes.clone(),parent_path.1].concat();
        }
    });

    (node_indexes,edge_indexes)
}

pub struct SimpleVisitor{

}

impl Visitor for  SimpleVisitor{

    fn visit_node(&mut self, node: &Node, parent_nodes: Vec<(&Node,&Transition)>,node_index:usize) {
        println!("node {node_index} {:?}",node);
        println!("parents {:?}",parent_nodes);
    }

}

#[test]
fn flow_test(){

    let file_path = "../../demo/data/flow/flow.json";

    let file = std::fs::File::open(&file_path).unwrap();

    let mut flow = serde_json::from_reader::<&std::fs::File, Flow>(&file).unwrap();
    flow.fill_dag();

    let sub_flow = flow.sub_flow("node4");

    println!("sub_flow : {:?}",sub_flow.unwrap().visit(&mut SimpleVisitor{}));

}

