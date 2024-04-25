use serde::{Serialize, Deserialize};
use crate::node::{ColumnMapping, DslNode};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SqlReader{

    id:Option<String>,

    uri:Option<String>,

    sql:String,

    #[serde(default = "default_protocol")]
    protocol:String,

    ///
    /// 列映射
    ///
    mapping:Option<Vec<ColumnMapping>>

}

///
/// sql protocol
///
fn default_protocol()->String{
    "binary".to_string()
}

impl DslNode for SqlReader {

    fn dsl(&self) -> Option<String> {
        match (&self.id,&self.uri) {
            (Some(id),Some(uri))=>{
                let mapping_dsl = match &self.mapping {
                    None => "".to_string(),
                    Some(mapping) => self.column_mapping_dsl(mapping)
                };
                Some(format!("let df_{} = read_sql(\"{}\",\"{}\",\"{}\"){}",id,uri,self.sql,self.protocol,mapping_dsl))
            }
            _=>None,
        }
    }
}