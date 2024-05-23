use serde::{Deserialize, Serialize};
use crate::style::pattern::ImagePattern;

///
/// declare type ZRColor = ColorString | LinearGradientObject | RadialGradientObject | PatternObject;
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZRColor{

    String(String),

    LinearGradient(LinearGradient),

    RadialGradient(RadialGradient),

    ImagePattern(ImagePattern)

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinearGradient{

    #[serde(flatten)]
    gradient:Gradient,

    #[serde(skip_serializing_if = "Option::is_none")]
    x:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x2:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y2:Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RadialGradient{
    #[serde(flatten)]
    gradient:Gradient,

    x:f64,
    y:f64,
    r:f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gradient{
    #[serde(skip_serializing_if = "Option::is_none")]
    id:Option<usize>,

    #[serde(rename = "type")]
    gradient_type:String,
    ///
    ///
    ///
    color_stops:Vec<GradientColorStop>,

    #[serde(skip_serializing_if = "Option::is_none")]
    global:Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GradientColorStop{
    offset:usize,

    color:String,
}