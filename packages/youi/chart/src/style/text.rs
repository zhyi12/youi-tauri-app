use serde::{Deserialize, Serialize};
use crate::data::OrdinalNumber;
use crate::OrdinalRawValue;
use crate::style::line::ZRLineType;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label{

    #[serde(flatten)]
    text:Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    show:Option<bool>,
    //position?: ElementTextConfig['position'];
    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset:Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_margin: Option<f32>,
    //     overflow?: TextStyleProps['overflow'];
    //     ellipsis?: TextStyleProps['ellipsis'];
    #[serde(skip_serializing_if = "Option::is_none")]
    silent:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    precision:Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_animation:Option<bool>,
    //     rich?: Dictionary<TextCommonOption>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text{
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_size:Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_height : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_type:Option<ZRLineType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_dash_offset: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<OrdinalNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<OrdinalNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_width: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_type: Option<ZRLineType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_dash_offset: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_blur: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_offset_x: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_offset_y: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}
///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle{
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_style: Option<String> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_weight: Option<String> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<String> ,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_height:Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_color:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_width:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_dash_offset: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_blur:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_offset_x:Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_offset_y:Option<f32>,

    ///
    /// 文字超出宽度是否截断或者换行。配置width时有效
    /// 'truncate' 截断，并在末尾显示ellipsis配置的文本，默认为...
    /// 'break' 换行
    /// 'breakAll' 换行，跟'break'不同的是，在英语等拉丁文中，'breakAll'还会强制单词内换行
    #[serde(skip_serializing_if = "Option::is_none")]
    overflow: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ellipsis: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color:Option<String>,
}