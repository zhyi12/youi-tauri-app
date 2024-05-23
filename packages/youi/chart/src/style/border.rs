use serde::{Deserialize, Serialize};

use crate::style::line::ZRLineType;
use crate::style::ZRColor;

///
/// 边框
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Border{
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color:Option<ZRColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_type:Option<ZRLineType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_cap:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_join:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_dash_offset:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_miter_limit:Option<f32>,
}

