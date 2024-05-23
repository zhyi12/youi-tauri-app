use serde::{Deserialize, Serialize};
use crate::data::{OrdinalNumber, OrdinalString};

///
/// 贴花
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decal{
    
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol:Option<OrdinalString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_keep_aspect:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dash_array_x:Option<Vec<OrdinalNumber>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dash_array_y:Option<OrdinalNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tile_width:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tile_height:Option<f32>,
}

// declare type DecalDashArrayX = number | (number | number[])[];
// declare type DecalDashArrayY = number | number[];
