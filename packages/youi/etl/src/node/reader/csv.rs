use std::fmt::Write;
use serde::{Serialize, Deserialize};
use crate::node::{ColumnMapping, DslNode};

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CsvReader{

    ///
    /// 唯一标识
    ///
    id:Option<String>,

    ///
    /// csv文件路径
    ///
    uri:Option<String>,

    ///
    /// 指定输出的csv列名、类型
    ///
    mapping:Option<Vec<ColumnMapping>>

}

impl CsvReader {
    ///
    ///
    ///
    pub fn set_id(&mut self,id:String){
        self.id = Some(id);
    }
}


impl DslNode for CsvReader {

    ///
    ///
    ///
    fn dsl(&self) -> Option<String> {
        match (&self.id,&self.uri) {
            (Some(id),Some(uri))=>{
                // 使用.fold替代.map(|_| format!(..)).collect::<String>().,
                // This allocates a new string for every element in the iterator
                let mut field_schemas = match &self.mapping {
                    None => "".to_string(),
                    Some(mapping) => mapping
                        .iter()
                        .fold(String::new(), |mut output, b| {
                            let _ = write!(output, "field(\"{}\",\"{}\"),",b.name,b.data_type);
                            output
                        })
                };
                let schema_param = if field_schemas.is_empty() {"".to_string()} else {
                    field_schemas.pop().unwrap();
                    format!(",[{}]",field_schemas)
                };

                Some(format!("let df_{} = read_csv(\"{}\"{})",id,uri,schema_param))
            }
            _=>None,
        }
    }
}