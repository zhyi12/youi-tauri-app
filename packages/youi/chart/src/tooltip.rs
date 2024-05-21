use serde::{Deserialize, Serialize};
use crate::style::TextStyle;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {
    position: Option<Vec<String>>,
    formatter: Option<String>,
    value_formatter: Option<String>,
    background_color: Option<String>,
    border_color: Option<String>,
    border_width: Option<usize>,
    padding: Option<usize>,
    text_style: Option<TextStyle>,
    extra_css_text: Option<String>,
}