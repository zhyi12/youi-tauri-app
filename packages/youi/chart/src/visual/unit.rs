use serde::{Serialize, Deserialize};
use crate::style::Decal;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualUnit{
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_alpha: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_lightness: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_saturation: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_hue: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decal: Option<Decal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lift_z: Option<f32>,
    
}
