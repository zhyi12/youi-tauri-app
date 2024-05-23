
use serde::{Deserialize, Serialize};

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternBase{
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<usize>,

    pattern_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_y: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagePattern{
    #[serde(flatten)]
    pattern:PatternBase,

    image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_height: Option<f64>,
}

// interface ImagePatternObject extends PatternObjectBase {

// }
// interface SVGPatternObject extends PatternObjectBase {
// svgElement: SVGVNode;
// svgWidth: number;
// svgHeight: number;
// }