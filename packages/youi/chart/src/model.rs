use polars_core::prelude::DataFrame;

use serde::{Deserialize, Serialize};
use crate::axis::Axis;
use crate::dataset::Dataset;

use crate::legend::Legend;
use crate::series::Series;
use crate::style::TextStyle;
use crate::title::Title;

///
/// 图表配置
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartOption{

    ///
    /// 标题
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    title:Option<Vec<Title>>,

    ///
    /// 图例
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    legend:Option<Vec<Legend>>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset:Option<Vec<Dataset>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis:Option<Axis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis:Option<Axis>,
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