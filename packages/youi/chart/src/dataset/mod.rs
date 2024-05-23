use std::collections::HashMap;
use polars_core::prelude::DataFrame;
use polars_io::{json::{JsonWriter,JsonFormat}, SerWriter};
use serde::{Serialize, Deserialize};

pub type RowData = HashMap<String,serde_json::Value>;

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset{

    #[serde(skip_serializing_if = "Option::is_none")]
    id:Option<String>,

    ///
    ///
    ///
    source:Vec<RowData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions:Option<Vec<String>>,

}

///
/// DataFrame 转 chart Dataset
///
impl From<&mut DataFrame> for Dataset {

    fn from(df: &mut DataFrame) -> Self {

        let mut json_buf = Vec::new();
        JsonWriter::new(&mut json_buf).with_json_format(JsonFormat::Json)
            .finish(df).expect("json write error");
        //
        let dimensions = df.get_column_names()
            .iter()
            .map(|s|s.to_string())
            .collect::<Vec<String>>();

        let source = serde_json::from_slice::<Vec<RowData>>(json_buf.as_slice()).unwrap();

        Self{
            id:None,
            source,
            dimensions:Some(dimensions)
        }
    }
}