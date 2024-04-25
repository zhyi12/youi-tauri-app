use itertools::Itertools;
use serde::{Deserialize,Serialize};
use crate::node::{CalculateColumn, DslNode};
use crate::NodeInput;

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithColumns{

    id:Option<String>,

    ///
    /// 输入
    ///
    inputs:Option<Vec<NodeInput>>,

    ///
    /// 计算列
    ///
    columns:Vec<CalculateColumn>
}


impl WithColumns {
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

impl DslNode for WithColumns {
    ///
    ///
    ///
    fn dsl(&self) -> Option<String> {
        match (&self.id,&self.inputs) {
            (Some(id),Some(inputs)) => {
                if inputs.is_empty() || inputs.len() != 1{
                    None
                }else{
                    let input = &inputs[0];
                    let columns_expr = self.columns.iter().map(|column|{
                        format!("{}.alias(\"{}\")",column.expression,column.name)
                    }).join(",");
                    Some(format!("let df_{} = df_{}.with_columns([{}])",id,input.id,columns_expr))
                }
            }
            _ => None,
        }
    }
}