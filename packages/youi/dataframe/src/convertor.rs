//!
//! AnyValue 转字符
//!
//!

use polars_core::prelude::AnyValue;

///
///
///
pub fn value_to_text(any:&AnyValue) -> String{
    
    match any {
        AnyValue::Boolean(x) => x.to_string(),
        AnyValue::String(x) => x.to_string(),
        AnyValue::UInt8(x) => format!("{}",x),
        AnyValue::UInt16(x) => format!("{}",x),
        AnyValue::UInt32(x) => format!("{}",x),
        AnyValue::UInt64(x) => format!("{}",x),
        AnyValue::Int8(x) => format!("{}",x),
        AnyValue::Int16(x) => format!("{}",x),
        AnyValue::Int32(x) => format!("{}",x),
        AnyValue::Int64(x) => format!("{}",x),
        AnyValue::Float32(x) => format!("{}",x),
        AnyValue::Float64(x) => format!("{}",x),
        AnyValue::Date(x) => format!("{}",x),
        AnyValue::Datetime(x, _y, _z) => format!("{}",x),
        AnyValue::Duration(x, _y) => format!("{}",x),
        AnyValue::Time(x) => format!("{}",x),
        AnyValue::String(x) =>  format!("{}",x),
        _=> "".to_string()
    }

}