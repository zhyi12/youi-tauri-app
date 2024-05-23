
use serde::{Deserialize, Serialize};

///
/// 折线图的高亮状态。
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Emphasis{

    ///
    /// 是否关闭高亮状态。
    /// 关闭高亮状态可以在鼠标移到图形上，tooltip 触发，或者图例联动的时候不再触发高亮效果。在图形非常多的时候可以关闭以提升交互流畅性。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled:Option<bool>,

    ///
    /// 在高亮图形时，是否淡出其它数据的图形已达到聚焦的效果。支持如下配置：
    /// 'none' 不淡出其它图形，默认使用该配置。
    /// 'self' 只聚焦（不淡出）当前高亮的数据的图形。
    /// 'series' 聚焦当前高亮的数据所在的系列的所有图形。
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    focus:Option<String>,
}