
use serde::{Deserialize, Serialize};
use crate::style::{Shadow, ZRColor};

///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AreaStyle{

    #[serde(skip_serializing_if = "Option::is_none")]
    color:Option<ZRColor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opacity:Option<f64>,

    #[serde(flatten)]
    shadow:Shadow
}