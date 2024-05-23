mod piecewise;
mod continuous;
mod unit;

use serde::{Serialize, Deserialize};
use crate::component::Component;
use crate::layout::BoxLayout;
use crate::series::SeriesIndex;
use crate::style::Border;
use crate::visual::continuous::ContinuousVisual;
use crate::visual::piecewise::PiecewiseVisual;

///
/// 视觉映射组件，用于进行『视觉编码』，也就是将数据映射到视觉元素（视觉通道）。
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
pub enum VisualMap{

    /// 连续型
    continuous(ContinuousVisual),

    /// 分段型
    piecewise(PiecewiseVisual)

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisualBase{

    #[serde(flatten)]
    component:Component,

    #[serde(flatten)]
    box_layout:BoxLayout,

    #[serde(flatten)]
    border:Border,
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realtime: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    series_index:Option<SeriesIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<f64>,
    //     /**
    //      * Visual configuration for the data in selection
    //      */
    //     inRange?: T;
    //     /**
    //      * Visual configuration for the out of selection
    //      */
    //     outOfRange?: T;
    //     controller?: {
    //         inRange?: T;
    //         outOfRange?: T;
    //     };
    //     target?: {
    //         inRange?: T;
    //         outOfRange?: T;
    //     };
    //     /**
    //      * Width of the display item
    //      */
    //     itemWidth?: number;
    //     /**
    //      * Height of the display item
    //      */
    //     itemHeight?: number;
    //     inverse?: boolean;
    //     orient?: 'horizontal' | 'vertical';
    //     backgroundColor?: ZRColor;
    //     contentColor?: ZRColor;
    //     inactiveColor?: ZRColor;
    //     /**
    //      * Padding of the component. Can be an array similar to CSS
    //      */
    //     padding?: number[] | number;
    //     /**
    //      * Gap between text and item
    //      */
    //     textGap?: number;
    //     precision?: number;
    //     /**
    //      * @deprecated
    //      * Option from version 2
    //      */
    //     color?: ColorString[];
    //     formatter?: string | LabelFormatter;
    //     /**
    //      * Text on the both end. Such as ['High', 'Low']
    //      */
    //     text?: string[];
    //     textStyle?: LabelOption;
    //     categories?: unknown;
}

//interface VisualMapOption<T extends VisualOptionBase = VisualOptionBase> extends ComponentOption, BoxLayoutOptionMixin, BorderOptionMixin