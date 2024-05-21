//!
//!
//!
use serde::{Serialize,Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_dash_offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_miter_limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_blur: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_x: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_y: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<usize>,
}