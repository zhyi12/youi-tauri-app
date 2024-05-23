use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

use crate::axis::AxisData;
use crate::OrdinalRawValue;

const FIELDS: &[&str] = &["value", "textStyle"];

enum Field { Value, TextStyle }

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`value` or `textStyle`")
            }

            fn visit_str<E>(self, value: &str) -> Result<Field, E>
                where
                    E: de::Error,
            {
                match value {
                    "value" => Ok(Field::Value),
                    "textStyle" => Ok(Field::TextStyle),
                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl<'de> Deserialize<'de> for AxisData {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {

        deserializer.deserialize_any(AxisDataVisitor)
        //deserializer.deserialize_struct("AxisData", FIELDS, AxisDataVisitor)

    }

}

struct AxisDataVisitor;

impl<'de> Visitor<'de> for AxisDataVisitor {

    type Value = AxisData;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct AxisData")
    }

    ///
    ///
    ///
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(AxisData{
            value: OrdinalRawValue::Number(v),
            text_style: None
        })
    }

    ///
    ///
    ///
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error{
        Ok(AxisData{
            value: OrdinalRawValue::String(v.to_string()),
            text_style: None
        })
    }

    ///
    ///
    ///
    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
    {
        let mut value = None;
        let mut text_style = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::Value => {
                    if value.is_some() {
                        return Err(de::Error::duplicate_field("value"));
                    }
                    value = Some(map.next_value()?);
                }
                Field::TextStyle => {
                    if text_style.is_some() {
                        return Err(de::Error::duplicate_field("textStyle"));
                    }
                    text_style = Some(map.next_value()?);
                }
            }
        }
        let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
        Ok(AxisData::new(value, text_style))
    }

}