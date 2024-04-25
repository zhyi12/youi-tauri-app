use itertools::Itertools;
use crate::node::DslNode;
use crate::NodeInput;
use serde::{Deserialize,Serialize};

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Union{

    id:Option<String>,
    ///
    /// 输入
    ///
    inputs:Option<Vec<NodeInput>>,
    ///
    /// union顺序
    ///
    ordered:Vec<String>,
    ///
    ///
    ///
    mapping:Vec<UnionMapping>
}

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnionMapping{

    ///
    /// 映射名
    ///
    name:Option<String>,

    ///
    /// 多输入对应的列名集合
    ///
    input_column_names:Vec<String>

}

impl UnionMapping {
    ///
    ///
    ///
    pub fn find_name(&self)->String{
        self.name.as_ref().unwrap_or(&"".to_string()).to_string()
    }

    ///
    ///
    ///
    pub fn find_input_column_name(&self,idx:usize)->String{
        (&self.input_column_names[idx]).to_string()
    }
}


impl Union {
    ///
    ///
    ///
    pub fn set_inputs(&mut self,inputs:Vec<NodeInput>){
        self.inputs = Some(inputs)
    }

    ///
    ///
    ///
    pub fn set_id(&mut self,id:String){
        self.id = Some(id);
    }
}

impl DslNode for Union {

    ///
    ///
    ///
    fn dsl(&self)->Option<String>{
        match (&self.id,&self.inputs) {
            (Some(id),Some(inputs)) => {
                let count = inputs.len();
                if count >0 {

                    let concat_expr = (0..count).map(|idx|{
                        let input_id = &self.ordered[idx];

                        let selects = self.mapping.iter().map(|m|{
                            let name = m.find_name();
                            let input_column_name = m.find_input_column_name(idx);
                            if name.as_str() == input_column_name.as_str() {
                                format!("col(\"{}\")",name)
                            }else{
                                format!("col(\"{}\").alias(\"{}\")",input_column_name,name)
                            }
                        }).join(",");

                        format!("df_{}.select([{}])",input_id,selects)
                    }).join(",");

                    Some(format!("let df_{} = concat([{}])",id,concat_expr))
                }else{
                    None
                }
            },
            _=>None
        }
    }
}