use serde::{Serialize,Deserialize};
use crate::series::{SeriesDatasetInfo, SeriesInfo, XyAxisSeriesInfo};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarSeries{

    #[serde(flatten)]
    info:SeriesInfo,

    #[serde(flatten)]
    axis:XyAxisSeriesInfo,

    #[serde(flatten)]
    dataset_info:SeriesDatasetInfo,
}