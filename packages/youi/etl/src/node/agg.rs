use itertools::Itertools;
use serde::{Serialize, Deserialize};
use crate::node::{DslNode};
use crate::NodeInput;

///
/// 汇总
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agg{

    id:Option<String>,

    ///
    /// 输入
    ///
    inputs:Option<Vec<NodeInput>>,

    ///
    /// 分组字段
    ///
    group_names:Vec<String>,

    ///
    /// 计量集合
    ///
    measures:Vec<Measure>
}

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measure{

    name:String,

    aggregate:String,

    alias:Option<String>
}

impl Measure {

    pub fn find_output_name(&self)->String{
        self.alias.as_ref().unwrap_or(&format!("{}_{}",self.name,self.aggregate)).to_string()
    }

}

impl Agg {

    ///
    ///
    ///
    pub fn set_id(&mut self,id:String){
        self.id = Some(id);
    }

    ///
    ///
    ///
    pub fn set_inputs(&mut self,inputs:Vec<NodeInput>){
        self.inputs = Some(inputs)
    }
}

impl DslNode for Agg {
    ///
    ///
    ///
    fn dsl(&self) -> Option<String> {
        match (&self.id,&self.inputs) {
            (Some(id),Some(inputs)) => {
                if inputs.len() == 1{
                    let input = &inputs[0].id;
                    let group_by = self.group_names.iter().map(|name|name.to_string()).join(",");
                    let measure_expr = self.measures.iter()
                        .map(|m|format!("col(\"{}\").{}().alias(\"{}\")",m.name,m.aggregate,m.find_output_name())).join(",");
                    Some(format!("let df_{} = df_{}.agg(\"{}\",[{}])",id,input,group_by,measure_expr))
                }else{
                   None
                }
            },
            _=>None
        }
    }
}
