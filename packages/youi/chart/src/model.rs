use polars_core::prelude::DataFrame;

use serde::{Deserialize, Serialize};
use crate::axis::Axis;
use crate::dataset::Dataset;

use crate::legend::Legend;
use crate::radar::Radar;
use crate::series::Series;
use crate::style::TextStyle;
use crate::title::Title;
use crate::tooltip::Tooltip;
use crate::de;
use crate::grid::Grid;
use crate::visual::VisualMap;

///
/// 图表配置
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartOption{

    ///
    /// 标题
    ///
    #[serde(skip_serializing_if = "Option::is_none",default="default_title",deserialize_with="de::vec_or_struct")]
    title:Option<Vec<Title>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip:Option<Tooltip>,
    ///
    /// 图例
    ///
    #[serde(skip_serializing_if = "Option::is_none",default="default_legend",deserialize_with="de::vec_or_struct")]
    legend:Option<Vec<Legend>>,

    #[serde(skip_serializing_if = "Option::is_none",default="default_grid",deserialize_with="de::vec_or_struct")]
    grid:Option<Vec<Grid>>,

    ///
    /// 调色盘颜色列表。如果系列没有设置颜色，则会依次循环从该列表中取颜色作为系列颜色。 默认为：
    /// ['#5470c6', '#91cc75', '#fac858', '#ee6666', '#73c0de', '#3ba272', '#fc8452', '#9a60b4', '#ea7ccc']
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    color:Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color:Option<String>,
    ///
    ///背景色，默认无背景。
    /// 支持使用rgb(255,255,255)，rgba(255,255,255,1)，#fff等方式设置为纯色，也支持设置为渐变色和纹理填充，具体见option.color
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    text_style:Option<TextStyle>,

    ///
    /// 序列
    ///
    series:Vec<Series>,

    #[serde(skip_serializing_if = "Option::is_none",default="default_dataset",deserialize_with="de::vec_or_struct")]
    dataset:Option<Vec<Dataset>>,

    #[serde(skip_serializing_if = "Option::is_none",default="default_visual_map",deserialize_with="de::vec_or_struct")]
    visual_map:Option<Vec<VisualMap>>,

    #[serde(skip_serializing_if = "Option::is_none",default="default_axis",deserialize_with="de::vec_or_struct")]
    x_axis:Option<Vec<Axis>>,

    #[serde(skip_serializing_if = "Option::is_none",default="default_axis",deserialize_with="de::vec_or_struct")]
    y_axis:Option<Vec<Axis>>,
    #[serde(skip_serializing_if = "Option::is_none",default="default_radar",deserialize_with="de::vec_or_struct")]
    radar:Option<Vec<Radar>>,
}

impl ChartOption {

    ///
    /// 添加dataFrame数据集
    ///
    pub fn add_dataset(&mut self,df:&mut DataFrame){
        let dataset = Dataset::from(df);
        if self.dataset.is_none(){
            self.dataset = Some(vec![]);
        }
        self.dataset.as_mut().unwrap().push(dataset);
    }
}

fn default_title()->Option<Vec<Title>>{
    None
}

fn default_legend()->Option<Vec<Legend>>{
    None
}

fn default_grid()->Option<Vec<Grid>>{
    None
}

fn default_dataset()->Option<Vec<Dataset>>{
    None
}

fn default_axis()->Option<Vec<Axis>>{
    None
}

fn default_radar()->Option<Vec<Radar>>{
    None
}

fn default_visual_map()->Option<Vec<VisualMap>>{
    None
}