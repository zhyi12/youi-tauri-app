use serde::{Serialize, Deserialize};
use crate::visual::unit::VisualUnit;
use crate::visual::VisualBase;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PiecewiseVisual{

    #[serde(flatten)]
    base:VisualBase,


    //align:'auto' | 'left' | 'right';
    //     minOpen:boolean;
    //     maxOpen:boolean;
    //     /**
    //      * When put the controller vertically, it is the length of
    //      * horizontal side of each item. Otherwise, vertical side.
    //      * When put the controller vertically, it is the length of
    //      * vertical side of each item. Otherwise, horizontal side.
    //      */
    //     itemWidth:Option<f32>,
    //     itemHeight:Option<f32>,
    //     itemSymbol:string;

    #[serde(skip_serializing_if = "Option::is_none")]
    pieces:Option<Vec<VisualPiece>>,


    //     /**
    //      * category names, like: ['some1', 'some2', 'some3'].
    //      * Attr min/max are ignored when categories set. See "Order Rule"
    //      */
    //     categories:string[];
    //     /**
    //      * If set to 5, auto split five pieces equally.
    //      * If set to 0 and component type not set, component type will be
    //      * determined as "continuous". (It is less reasonable but for ec2
    //      * compatibility, see echarts/component/visualMap/typeDefaulter)
    //      */
    //     splitNumber:Option<f32>,
    //     /**
    //      * Object. If not specified, means selected. When pieces and splitNumber: {'0': true, '5': true}
    //      * When categories: {'cate1': false, 'cate3': true} When selected === false, means all unselected.
    //      */
    //     selected:Dictionary<boolean>;
    //     selectedMode:'multiple' | 'single' | boolean;
    //     /**
    //      * By default, when text is used, label will hide (the logic
    //      * is remained for compatibility reason)
    //      */
    //     showLabel:boolean;
    //     itemGap:Option<f32>,
    //     hoverLink:boolean;

}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualPiece{

    #[serde(flatten)]
    unit:VisualUnit,
    #[serde(skip_serializing_if = "Option::is_none")]
    min:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lt:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gt:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label:Option<String>,
}