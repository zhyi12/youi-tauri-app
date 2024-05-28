use serde::{Serialize, Deserialize};
use crate::OrdinalRawValue;
use crate::style::ItemStyle;
use crate::visual::VisualBase;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousVisual {
    #[serde(flatten)]
    base: VisualBase,

    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calculable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_link_data_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_link_on_handle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle_size: Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle_style: Option<ItemStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indicator_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indicator_size: Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indicator_style: Option<ItemStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emphasis{
    #[serde(skip_serializing_if = "Option::is_none")]
    handle_style:Option<ItemStyle>,
}