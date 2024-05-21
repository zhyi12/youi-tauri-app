use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    animation: bool,
    animation_threshold: Option<usize>,
    animation_duration: Option<usize>,
    animation_easing: Option<String>,
    animation_delay: Option<usize>,
    animation_duration_update: Option<usize>,
    animation_easing_update: Option<String>,
    animation_delay_update: Option<usize>,
}