use std::marker::PhantomData;
use serde::{Deserializer};
use serde::de::{Error, SeqAccess, Visitor};
use crate::OrdinalRawValue;
use crate::style::Gap;

pub fn gap_deserialize<'de, D>(deserializer: D) -> Result<Option<Gap>, D::Error>
    where
        D: Deserializer<'de>
{
    struct GapStruct(PhantomData<fn() -> Gap>);

    impl<'de> Visitor<'de> for GapStruct {
        type Value = Option<Gap>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("gap")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: Error {
            Ok(Some(Gap::Bool(v)))
        }

        ///
        /// 序列
        ///
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>, {

            let value1: Option<OrdinalRawValue> = seq.next_element()?;
            let value2: Option<OrdinalRawValue> = seq.next_element()?;

            Ok(Some(Gap::Value(value1.unwrap(),value2.unwrap())))
        }

    }

    deserializer.deserialize_any(GapStruct(PhantomData))
}