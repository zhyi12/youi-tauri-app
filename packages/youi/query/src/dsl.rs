use itertools::Itertools;
use std::fmt::Write;
use crate::column::{Column, Columns};
use crate::error::{QueryError, QueryResult as Result};
use crate::QueryModel;

pub trait DslNode{

    ///
    /// 转dsl脚本
    ///
    fn to_dsl(&self,input_step_id:Option<&str>)->Result<String>;

    fn build_df_id(&self,id:&str)->String{
        format!("df_{}",id.replace("-","_"))
    }

    ///
    /// 检查输入
    ///
    fn check_input(&self,input_step_id:Option<&str>,message:&str)->Result<()>{
        if input_step_id.is_none(){
            return Err(QueryError::NeedPrevStep(message.to_string()));
        }
        Ok(())
    }
    ///
    /// 列转 dsl col表达式集合
    ///
    fn to_columns_dsl(&self,columns:Vec<&Column>)->Result<String>{
        let mut dsl = columns
            .iter()
            .fold(String::new(),|mut output,c|{
                let _ = match &c.expression {
                    None => write!(output,"col(\"{}\"),",c.name),
                    Some(e) => write!(output,"({}).alias(\"{}\"),",e,c.name)
                };
                output
            });
        dsl.pop();
        Ok(dsl)
    }

}

///
///
///
impl DslNode for QueryModel {
    ///
    /// 输出dsl
    ///
    fn to_dsl(&self, _df_name: Option<&str>) -> Result<String> {
        let mut input_id:Option<&str> = None;

        let dsl = (0..self.steps.len()).map(|idx|{
            let dsl = self.steps[idx].to_dsl(input_id).unwrap();
            if !dsl.is_empty(){
                input_id = Some(self.steps[idx].get_id());
            }
            dsl
        }).filter(|s|!s.is_empty())
            .join(";\n");

        Ok(format!("{};\n{}",dsl,self.build_df_id(input_id.unwrap())))
    }
}
