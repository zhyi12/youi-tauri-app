
use serde::{Deserialize, Serialize};
use crate::style::{Shadow, ZRColor};

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle{

    #[serde(flatten)]
    shadow:Shadow,
    #[serde(skip_serializing_if = "Option::is_none")]
    color:Option<ZRColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dash_offset:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    miter_limit:Option<f32>,
    #[serde(rename="type",skip_serializing_if = "Option::is_none")]
    line_type:Option<ZRLineType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    join: Option<String>,
}

///
/// declare type ZRLineType = 'solid' | 'dotted' | 'dashed' | number | number[];
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZRLineType{

    String(String),

    Number(f32),

    Numbers(Vec<f32>)

}