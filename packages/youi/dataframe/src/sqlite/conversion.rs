use polars_core::prelude::DataType;

#[derive(Debug)]
#[repr(transparent)]
pub struct DfDataType<T>(pub T);

impl<T> Clone for DfDataType<T>
    where
        T: Clone,
{
    fn clone(&self) -> Self {
        DfDataType(self.0.clone())
    }
}
impl<T> From<T> for DfDataType<T> {
    fn from(t: T) -> Self {
        DfDataType(t)
    }
}

impl From<&str> for DfDataType<DataType> {
    fn from(str: &str) -> Self {
        let dt = match str {
            "BOOL" | "BOOLEAN" =>{
                DataType::Boolean
            }
            "INT" | "NUMBER" | "INTEGER" | "BIGINT" | "INT8"=>{
                DataType::Int64
            }
            "VARCHAR" | "STRING" | "TEXT"=>{
                DataType::String
            }
            "REAL"=>{
                DataType::Float64
            }
            _=>{
                DataType::String
            }
        };
        DfDataType(dt)
    }
}
