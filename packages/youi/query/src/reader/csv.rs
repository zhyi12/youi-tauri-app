
use std::fmt::Write;
use serde::{Serialize, Deserialize};
use crate::column::Columns;
use crate::dsl::DslNode;

#[derive(Serialize, Deserialize,Debug)]
pub struct CsvReader{

    uri:String,

    ///
    /// csv 文件编码
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    encode:Option<String>,

    #[serde(flatten)]
    pub(crate) columns_info:Columns
}

impl DslNode for CsvReader{

    fn to_dsl(&self, _input_step_id: Option<&str>) -> crate::error::QueryResult<String> {

        let selected_columns = self.columns_info.find_selected_columns();

        let mut field_schemas = selected_columns
            .iter()
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "field(\"{}\",\"{}\"),",b.name,
                               b.data_type.as_ref().unwrap_or(&"text".to_string()));
                output
            });

        let schema_param = if field_schemas.is_empty() {"".to_string()} else {
            field_schemas.pop().unwrap();
            format!(",[{}]",field_schemas)
        };

        Ok(format!("read_csv(\"{}\"{}).select([{}])",
                   self.uri,schema_param,self.to_columns_dsl(selected_columns)?))
    }
}