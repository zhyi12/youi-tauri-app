use serde::{Deserialize, Serialize};
///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shadow{
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_blur:Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_y: Option<f64>,
}