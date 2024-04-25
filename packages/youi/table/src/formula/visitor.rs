use hashbrown::HashMap;
use itertools::Itertools;
use youi_flow::{Visitor};
use youi_flow::node::Node;
use youi_flow::transition::Transition;

///
/// 公式遍历，生成顺序执行的脚本
///
pub(crate) struct FormulaFlowVisitor{
    ///
    /// 公式批次 <ID,(序号，批次)>
    ///
    formula_batches:HashMap<String,(usize,usize)>,
}

impl FormulaFlowVisitor{
    pub fn new()->Self{
        Self{
            formula_batches:HashMap::new()
        }
    }
}

impl Visitor for FormulaFlowVisitor{

    fn visit_node(&mut self, node: &Node, parents: Vec<(&Node, &Transition)>,node_index:usize) {
        if !parents.is_empty(){
            if is_root((&parents[0]).0){
                self.formula_batches.insert(node.id.to_string(),(node_index,0));
            }else{
                let batch = parents.iter().map(|(p,_)|{
                    let opt = (&self.formula_batches).get(&p.id);
                    match opt {
                        Some(bt)=>bt.1+1,
                        None=>0
                    }
                }).max().unwrap();

                self.formula_batches.insert(node.id.to_string(),(node_index,batch));
            }
        }
    }

}

impl FormulaFlowVisitor {

    ///
    ///
    ///
    pub fn find_batch_indexes(&self)->Vec<Vec<usize>>{
        let grouped = self.formula_batches
            .iter()
            .map(|(_id,(index,batch))|(*batch,*index))
            .into_group_map();

        grouped.keys().sorted().map(|key|{
            grouped.get(key).unwrap().clone()
        }).collect::<Vec<Vec<usize>>>()

    }
}
///
///
///
fn is_root(node: &Node)->bool{
    node.id.as_str() == "start"
}
