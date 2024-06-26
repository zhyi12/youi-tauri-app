use serde::{Serialize, Deserialize};

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    #[serde(skip_serializing_if = "Option::is_none")]
    animation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_threshold: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_duration: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_easing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_delay: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_duration_update: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_easing_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_delay_update: Option<usize>,
}