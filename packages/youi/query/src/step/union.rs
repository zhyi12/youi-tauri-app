use std::fmt::Write;
use serde::{Serialize, Deserialize};
use crate::dsl::DslNode;
use crate::reader::Reader;
use crate::step::StepInfo;
use crate::error::QueryResult as Result;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct Union{
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,

    ///
    /// 合并集合
    ///
    readers:Option<Vec<Reader>>,

    ///
    /// 合并输出列
    ///
    union_columns:Option<Vec<UnionColumn>>,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
struct UnionColumn{

    ///
    /// 输出列名
    ///
    name:String,

    ///
    /// 输入列名
    ///
    input_name:String,

    ///
    ///
    ///
    data_type:String,

    ///
    /// 合并的列名称
    ///
    union_names:Vec<String>
}

///
///
///
impl DslNode for Union{

    ///
    ///
    ///
    fn to_dsl(&self,input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        // 检查输入
        self.check_input(input_step_id,self.info.text.as_ref().unwrap_or(&"Union".to_string()))?;

        Ok(UnionDslBuilder::from(self).finish(input_step_id.unwrap())?)
    }
}

struct UnionDslBuilder<'a>{
    union:&'a Union,
    dsl:String,
}

impl<'a> From<&'a Union> for UnionDslBuilder<'a>{
    fn from(union: &'a Union) -> Self {
        Self{
            union,
            dsl:String::new()
        }
    }
}

impl<'a> UnionDslBuilder<'a> {
    pub fn finish(&mut self,input_step_id:&str)->Result<String> {
        self.dsl = String::new();

        match (&self.union.union_columns,&self.union.readers) {
            (Some(union_columns),Some(readers))=>{
                write!(self.dsl,"let {} = concat([{}",self.union.build_df_id(&self.union.info.id),self.union.build_df_id(input_step_id))?;
                self.write_input_with_columns(union_columns)?;
                self.write_readers(union_columns,readers)?;
                write!(self.dsl,"])")?;
            },
            (_, _) => {}
        }
        Ok(self.dsl.to_string())
    }

    ///
    /// 输入自动映射
    ///
    pub fn write_input_with_columns(&mut self,union_columns:&Vec<UnionColumn>)->Result<()>{
        let input_select_expr = union_columns.iter()
            .fold(String::new(),|mut output,c|{
                write!(output,"{}",build_select_column(&c.input_name,&c.name,&c.data_type).unwrap()).unwrap();
                output
            });
        write!(self.dsl,".select([{}])",input_select_expr)?;
        Ok(())
    }

    pub fn write_readers(&mut self,union_columns:&Vec<UnionColumn>,readers:&Vec<Reader>)->Result<()>{
        let expr = readers
            .iter()
            .zip(0..readers.len())
            .fold(String::new(),|mut output,(reader,idx)|{
                let reade_dsl = reader.to_dsl(None).unwrap();
                write!(output,",{}",reade_dsl).unwrap();
                write!(output,".select([{}])",build_select_columns(idx,union_columns).unwrap()).unwrap();
                output
            });
        write!(self.dsl,"{}",expr)?;
        Ok(())
    }
}

///
///
///
fn build_select_columns(idx:usize,union_columns:&Vec<UnionColumn>)->Result<String>{
    Ok(union_columns
        .iter()
        .fold(String::new(),|mut output,uc|{
            let union_name = &uc.union_names[idx];
            write!(output,"{}",build_select_column(union_name,&uc.name,&uc.data_type).unwrap()).unwrap();
            output
        })
    )
}

fn build_select_column(union_name:&str,name:&str,data_type:&str)->Result<String>{
    let mut output = String::new();
    if union_name.is_empty(){
        if data_type == "text" || data_type == "str"{
            write!(output,"lit(\"\")").unwrap();
        }else{
            write!(output,"lit(0)").unwrap();
        }
    }else{
        write!(output,"col(\"{}\")",union_name).unwrap();
    }
    if name != union_name{
        write!(output,".alias(\"{}\")",name).unwrap();
    }
    write!(output,",").unwrap();

    Ok(output)
}