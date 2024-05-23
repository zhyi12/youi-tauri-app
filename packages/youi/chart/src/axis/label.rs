use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::style::Text;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisLabel{

    #[serde(flatten)]
    text:Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    show:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inside:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotate:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_min_label:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_max_label:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align_min_label:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align_max_label:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align_min_label:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align_max_label:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    margin:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rich:Option<HashMap<String,Text>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_overlap:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overflow:Option<String>,
}