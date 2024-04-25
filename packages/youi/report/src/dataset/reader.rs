use serde::{Serialize,Deserialize};
use crate::DataType;

///
/// csv
/// sql
/// parquet
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataReader{
    ///
    /// read_csv read_sql read_parquet
    ///
    reader:String,

    ///
    ///
    ///
    uri:String,

    ///
    ///
    ///
    reader_params:Option<Vec<DataType>>,
}

impl DataReader {

    pub fn reader(&self)->&str{
        &self.reader
    }

    pub fn uri(&self)->&str{
        &self.uri
    }

    pub fn reader_params(&self) -> Option<&Vec<DataType>> {
        self.reader_params.as_ref()
    }
}
