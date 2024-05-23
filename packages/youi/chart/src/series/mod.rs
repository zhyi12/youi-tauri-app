//!
//! 序列
//!
//!

mod line;
mod bar;
mod pie;
mod radar;
mod stack;

use serde::{Serialize,Deserialize};
use bar::BarSeries;
use line::LineSeries;
use pie::PieSeries;
use crate::data::OrdinalData;
use crate::series::radar::RadarSeries;
use crate::style::Label;
use crate::tooltip::Tooltip;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
pub enum Series{
    ///
    ///
    ///
    line(LineSeries),
    ///
    ///
    ///
    bar(BarSeries),
    ///
    ///
    ///
    pie(PieSeries),

    ///
    ///
    ///
    radar(RadarSeries),
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesInfo{
    #[serde(skip_serializing_if = "Option::is_none")]
    id:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_by:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system :Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel:Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z:Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    silent:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip:Option<Tooltip>,
    ///
    ///
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    data:Option<OrdinalData>,

}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XyAxisSeriesInfo{
    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index:Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index:Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesDatasetInfo{

    ///
    ///使用 dimensions 定义 series.data 或者 dataset.source 的每个维度的信息。
    /// 注意：如果使用了 dataset，那么可以在 dataset.dimensions 中定义 dimension ，
    /// 或者在 dataset.source 的第一行/列中给出 dimension 名称。于是就不用在这里指定 dimension。
    /// 但如果在这里指定了 dimensions，那么优先使用这里的。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions:Option<Vec<String>>,

    ///
    /// 如果 series.data 没有指定，并且 dataset 存在，那么就会使用 dataset。datasetIndex 指定本系列使用那个 dataset。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_index:Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesLabel{
    ///
    ///
    ///
    formatter:Option<String>,

    #[serde(flatten)]
    label:Label,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineEndLabel{
    #[serde(flatten)]
    label:SeriesLabel,

    value_animation:Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SeriesIndex{
    All(String),

    Index(f32),

    Indexed(Vec<f32>)

}