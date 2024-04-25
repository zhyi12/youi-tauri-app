use serde::{Serialize,Deserialize};
use crate::node::{ColumnMapping, DslNode, InputtingAble};
use crate::NodeInput;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Join{

    id:Option<String>,
    ///
    /// 输入
    ///
    inputs:Option<Vec<NodeInput>>,

    left:String,

    right:String,

    ///
    /// 左边连接字段
    ///
    left_on:String,

    ///
    /// 右边连接字段
    ///
    right_on:String,

    ///
    /// 列映射
    ///
    mapping:Option<Vec<ColumnMapping>>
}

impl Join {
    ///
    ///
    ///
    pub fn set_id(&mut self,id:String){
        self.id = Some(id);
    }
}

impl InputtingAble for Join {
    ///
    ///
    ///
    fn set_inputs(&mut self,inputs:Vec<NodeInput>){
        self.inputs = Some(inputs)
    }

}

impl DslNode for Join {

    ///
    ///
    ///
    fn dsl(&self)->Option<String>{
        println!("{:?}",self);
        match (&self.id,&self.inputs) {
            (Some(id),Some(inputs)) => {
                if inputs.len() == 2{
                    let mapping_dsl = match &self.mapping {
                        None => "".to_string(),
                        Some(mapping) => self.column_mapping_dsl(mapping)
                    };

                    Some(format!("let df_{} = df_{}.left_join(df_{},\"{}\",\"{}\"){}",id,
                                 self.left,self.right,self.left_on,self.right_on,mapping_dsl))
                }else{
                    None
                }
            },
            _=>None
        }
    }
}