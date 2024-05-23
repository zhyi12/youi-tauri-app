use serde::{Serialize, Deserialize};

use crate::axis::AxisBase;
use crate::OrdinalRawValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericAxis{
    
    #[serde(flatten)]
    axis:AxisBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<Vec<OrdinalRawValue>>,
    ///
    /// AxisTick and axisLabel and splitLine are calculated based on splitNumber.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    split_number: Option<f32>,
    /// 
    /// Interval specifies the span of the ticks is mandatorily.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<f32>,
    ///
    ///  Specify min interval when auto calculate tick interval.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    min_interval: Option<f32>,
    ///
    ///  Specify max interval when auto calculate tick interval.
    /// 
    #[serde(skip_serializing_if = "Option::is_none")]
    max_interval: Option<f32>,
    ///
    /// If align ticks to the first axis that is not use alignTicks
    /// If all axes has alignTicks: true. The first one will be applied.
    /// Will be ignored if interval is set.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    align_ticks: Option<bool>
}