
use serde::{Deserialize, Serialize};
///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle{

    color:Option<String>,
    width:Option<f32>,



    // color ,
     //    width ,
     //    type ,
     //    dashOffset , cap , join , miterLimit , shadowBlur , shadowColor , shadowOffsetX , shadowOffsetY , opacity }
}