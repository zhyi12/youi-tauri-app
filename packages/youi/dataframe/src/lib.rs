use polars_core::prelude::DataFrame;
use polars_io::prelude::{JsonFormat, JsonWriter};
use polars_io::SerWriter;
pub mod lazy;
pub mod cube;

pub mod convertor;
pub mod error;

#[cfg(feature = "search_dataframe")]
pub mod column_match;

#[cfg(feature = "sqlite_dataframe")]
pub mod sqlite;

#[cfg(feature = "xls_dataframe")]
mod xls;

#[cfg(feature = "geo_dataframe")]
mod geo;

pub struct ItemMapping{
    pub name:String,
    pub mapping_name:Option<String>
}

///
/// 字段
///
pub struct DataField{
    pub name:String,
    pub data_type:String
}

///
///
///
pub fn df_to_json(mut df:DataFrame)->String{
    let mut json_buf = Vec::new();
    //将dataFrame写入Vec
    JsonWriter::new(&mut json_buf).with_json_format(JsonFormat::Json)
        .finish(&mut df).expect("json write error");

    //转换为String对象
    let json_str = String::from_utf8(json_buf).unwrap();
    json_str
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}