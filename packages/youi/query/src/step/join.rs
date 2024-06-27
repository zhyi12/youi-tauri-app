use itertools::Itertools;
use std::fmt::Write;
use serde::{Serialize, Deserialize};
use crate::dsl::DslNode;
use crate::error::QueryResult as Result;
use crate::reader::Reader;
use crate::step::StepInfo;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Join{

    #[serde(flatten)]
    reader:Reader,
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,
    ///
    /// 连接方式，left、right、outer、inner
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    how:Option<String>,

    ///
    /// 左边连接字段集合
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) join_columns:Option<Vec<JoinColumn>>,

}

fn default_how()->String{
    "left".to_string()
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinColumn{

    pub(crate) name:String,

    pub(crate) left:String,

    pub(crate) right:String
}

///
///
///
impl DslNode for Join {

    ///
    ///
    ///
    fn to_dsl(&self,input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        // 检查输入
        self.check_input(input_step_id,self.info.text.as_ref().unwrap_or(&"Join".to_string()))?;

        Ok(JoinDslBuilder::from(self).finish(input_step_id.unwrap())?)
    }
}

struct JoinDslBuilder<'a>{
    join:&'a Join,
    dsl:String
}

impl<'a> From<&'a Join> for JoinDslBuilder<'a>{

    fn from(join: &'a Join) -> Self {
        Self{
            join,
            dsl:String::new()
        }
    }
}

impl<'a> JoinDslBuilder<'a> {

    ///
    ///
    ///
    pub fn finish(&mut self,input_step_id:&str)->Result<String>{
        self.dsl = String::new();

        match (&self.join.how,&self.join.join_columns) {
            (Some(how), Some(join_columns)) => {
                // 判断是否右连接
                let is_right_join = match how.as_str() {
                    "right"=>true,
                    _=>false
                };

                let how = if is_right_join {
                    // 右连接方式转换为左连接，df_right.join(df_left,...)
                    "left".to_string()
                }else{
                    self.join.how.as_ref().unwrap_or(&"left".to_string()).to_string()
                };

                let df_id = &self.join.build_df_id(&self.join.info.id);
                let input_df_id = &self.join.build_df_id(input_step_id);
                let reader_dsl = &self.join.reader.to_dsl(None)?;

                let on = join_columns.iter().map(|c|&c.name).join(",");

                write!(&mut self.dsl,"let {} = ",df_id)?;

                self.write_left(if is_right_join { reader_dsl} else {input_df_id},join_columns,is_right_join)?;

                write!(&mut self.dsl,".join(")?;
                self.write_right(if is_right_join {input_df_id} else {reader_dsl},join_columns,is_right_join,&how == "outer")?;

                write!(&mut self.dsl,",\"{}\",\"{}\",\"{1}\")",how,on)?;

            },
            _=>{}
        };

        Ok(self.dsl.to_string())
    }

    ///
    ///
    ///
    fn write_left(&mut self,left_dsl:&str,join_columns:&Vec<JoinColumn>,is_right_join:bool)->Result<()>{
        //dsl
        write!(&mut self.dsl,"{}",left_dsl)?;

        let mut left_width_columns = join_columns
            .iter()
            .fold(String::new(),|mut output,c|{
                let left_col = if is_right_join {&c.right} else {&c.left};
                if left_col.as_str() != c.name.as_str(){
                    write!(output,"col(\"{}\").alias(\"{}\"),",left_col,c.name).unwrap();
                }
                output
            });

        if !left_width_columns.is_empty(){
            left_width_columns.pop();
            write!(&mut self.dsl,".with_columns([{}])",left_width_columns)?;
        }

        Ok(())
    }

    fn write_right(&mut self,right_dsl:&str,join_columns:&Vec<JoinColumn>,is_right_join:bool,is_outer:bool)->Result<()>{

        write!(&mut self.dsl,"{}",right_dsl)?;

        let mut right_width_columns = join_columns
            .iter()
            .fold(String::new(),|mut output,c|{
                let right_col = if is_right_join {&c.left} else {&c.right};
                let out = if right_col.as_str() == c.name.as_str() {
                    if is_outer{
                         ""
                    }else{
                        &format!("{}_right",right_col)
                    }
                } else {
                    &c.name
                };

                if !out.is_empty(){
                    write!(output,"col(\"{}\").alias(\"{}\"),",right_col,out).unwrap();
                }

                output
            });

        if !right_width_columns.is_empty(){
            right_width_columns.pop();
            write!(&mut self.dsl,".with_columns([{}])",right_width_columns)?;
        }

        Ok(())
    }
}