use crate::error::QueryResult as Result;

pub trait SqlNode{

    ///
    ///
    ///
    fn to_sql(&self)->Result<String>;

}