use serde::{Serialize,Deserialize};
use crate::dsl::DslNode;
use crate::reader::csv::CsvReader;
use crate::reader::db::DbReader;

mod csv;
mod db;

#[derive(Serialize, Deserialize,Debug)]
#[serde(tag = "reader")]
pub enum Reader{

    CsvReader(CsvReader),

    DbReader(DbReader),

}

impl DslNode for Reader {

    fn to_dsl(&self, input_step_id: Option<&str>) -> crate::error::QueryResult<String> {
        match self {
            Reader::CsvReader(r) => r.to_dsl(input_step_id),
            Reader::DbReader(r) => r.to_dsl(input_step_id)
        }
    }
}