use std::collections::HashMap;
use polars_core::prelude::DataFrame;
use polars_io::{json::{JsonWriter,JsonFormat}, SerWriter};
use serde::{Serialize, Deserialize};
use crate::OrdinalRawValue;
use crate::transform::DataTransform;
use crate::de;

pub type RowData = HashMap<String,OrdinalRawValue>;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    source:Option<Vec<Row>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions:Option<Vec<String>>,

    ///
    /// 数据集转换
    ///
    #[serde(skip_serializing_if = "Option::is_none",default="default_transform",deserialize_with="de::vec_or_struct")]
    transform:Option<Vec<DataTransform>>
}


fn default_transform()->Option<Vec<DataTransform>>{
    None
}



#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Row{

    Map(RowData),

    Vec(Vec<OrdinalRawValue>)

}

///
///
///
impl From<&mut DataFrame> for Dataset {
    fn from(df: &mut DataFrame) -> Self {
        let mut dataset = Self{
            id:None,
            source: None,
            dimensions: None,
            transform: None
        };
        dataset.set_source_by_df(df);
        dataset
    }
}

impl Dataset {
    ///
    /// 设置source
    ///
    pub fn set_source_by_df(&mut self,df: &mut DataFrame){
        //
        let mut json_buf = Vec::new();
        JsonWriter::new(&mut json_buf).with_json_format(JsonFormat::Json)
            .finish(df).expect("json write error");
        //
        let dimensions = df.get_column_names()
            .iter()
            .map(|s|s.to_string())
            .collect::<Vec<String>>();

        self.source = Some(serde_json::from_slice::<Vec<Row>>(json_buf.as_slice()).unwrap());
        self.dimensions = Some(dimensions);

    }
}