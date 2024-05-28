
mod filter;
mod sort;
mod regression;

use serde::{Serialize, Deserialize};
use crate::transform::filter::FilterConfig;
use crate::transform::regression::RegressionConfig;
use crate::transform::sort::SortConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataTransform{
    ///
    ///
    ///
    #[serde(rename = "type")]
    transform_type:String,

    config:DataTransformConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataTransformConfig{

    Filter(FilterConfig),

    Sort(SortConfig),

    Regression(RegressionConfig),

}
