//!
//!
//!
//!
mod axis;
mod gap;
mod data;

use std::marker::PhantomData;
use serde::{Deserialize, Deserializer};
use serde::de::{MapAccess, SeqAccess, Visitor};

pub use gap::gap_deserialize;

///
/// json的序列或者结构体反序列化为集合属性
///
pub fn vec_or_struct<'de, T, D>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T> where T: Deserialize<'de> {
        type Value = Option<Vec<T>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct or struct-vec")
        }

        ///
        /// 序列
        ///
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>, {
            let mut ts: Vec<T> = vec![];

            let mut elem: std::option::Option<T> = seq.next_element()?;
            while elem.is_some() {
                ts.push(elem.unwrap());
                elem = seq.next_element()?;
            }

            Ok(Some(ts))
        }

        ///
        /// 结构体
        ///
        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
            let t = T::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(vec![t]))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}