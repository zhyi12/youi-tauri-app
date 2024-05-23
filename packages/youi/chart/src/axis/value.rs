use serde::{Serialize, Deserialize};
use crate::axis::label::AxisLabel;
use crate::axis::numeric::NumericAxis;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueAxis{

    #[serde(flatten)]
    axis:NumericAxis,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale:Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label:Option<AxisLabel>,

}