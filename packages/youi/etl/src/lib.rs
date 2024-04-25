mod node;

use std::collections::HashMap;
use itertools::Itertools;
use log::debug;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use youi_flow::{Flow, Visitor};
use youi_flow::node::Node;
use youi_flow::transition::Transition;

///
///
///
pub struct EtlExecutor<'a>{
    flow:&'a mut Flow
}

impl<'a> From<&'a mut Flow> for EtlExecutor<'a>{
    ///
    ///
    ///
    fn from(flow: &'a mut Flow) -> Self {
        //初始化有向无环图
        flow.fill_dag();
        Self{
            flow
        }
    }
}

impl<'a> EtlExecutor<'a> {

    ///
    /// 按步骤执行etl
    ///
    pub fn run_step(&self,step_id:&str)->Option<String>{
        let sub_flow = self.flow.sub_flow(step_id);
        match sub_flow {
            None => None,
            Some(sub) => {
                let mut visitor = EtlVisitor{dsl:vec![]};
                sub.visit(&mut visitor);
                Some(format!("{};\ndf_{}",visitor.dsl.iter().join(";\n"),step_id))
            }
        }
    }
}

///
///
///
pub fn to_dsl(flow:&mut Flow,node_id:&str)->Option<String>{
    EtlExecutor::from(flow).run_step(node_id)
}

///
/// 输入
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInput{
    id:String,
    expression:Option<String>
}
///
///
///
pub struct EtlVisitor{
    dsl:Vec<String>
}


impl Visitor for EtlVisitor {

    ///
    /// 遍历节点
    ///
    fn visit_node(&mut self, node: &Node, parents: Vec<(&Node, &Transition)>,_node_index:usize) {
        match &node.node_object {
            Some(obj)=>{
                let opt_name = obj.get("name");
                debug!("opt_name {:?}",opt_name);
                match opt_name {
                    None => {}
                    Some(value) => {
                        match value {
                            Value::String(_) => self.node_dsl(node.id.as_str(),obj,parents),
                            _=>{}
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

impl EtlVisitor {

    ///
    ///
    ///
    pub fn node_dsl(&mut self,node_id:&str,node_object: &HashMap<String,Value>, parents: Vec<(&Node, &Transition)>){
        let object_json = serde_json::to_string(node_object).unwrap();
        let result_object_node = serde_json::from_str::<crate::node::Node>(&object_json);

        let inputs = parents.iter().map(|(node,..)|{
            NodeInput{
                id: node.id.clone(),
                expression: None
            }
        }).collect::<Vec<NodeInput>>();

        match result_object_node {
            Ok(mut object_node) => {
                //设置inputs
                object_node.set_id(node_id.to_string());
                object_node.set_inputs(inputs);

                let opt_dsl = object_node.dsl();

                match opt_dsl {
                    None => {},
                    Some(dsl)=>{
                        self.dsl.push(dsl);
                    }
                }
            }
            Err(err) => {
                println!("{:?}",err);
            }
        }
    }
}

#[test]
fn flow_test(){

    let file_path = "../../demo/data/flow/flow.json";

    let file = std::fs::File::open(&file_path).unwrap();

    let mut flow = serde_json::from_reader::<&std::fs::File, Flow>(&file).unwrap();

    let dsl = to_dsl(&mut flow,"node9");

    println!("dsl: {}",dsl.unwrap());

}