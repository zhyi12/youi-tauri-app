use serde::{Deserialize, Serialize};
///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle{

    color: String,
    font_style: Option<String> ,
    font_weight: Option<String> ,
    font_family: Option<String> ,
    font_size: Option<usize>,
    line_height:Option<usize>,
    width:Option<f32>,
    height:Option<f32>,
    text_border_color:Option<String>,
    text_border_width:Option<f32>,
    text_border_type: Option<String>,
    text_border_dash_offset: Option<f32>,
    text_shadow_color: Option<String>,
    text_shadow_blur:Option<f32>,
    text_shadow_offset_x:Option<f32>,
    text_shadow_offset_y:Option<f32>,

    ///
    /// 文字超出宽度是否截断或者换行。配置width时有效
    /// 'truncate' 截断，并在末尾显示ellipsis配置的文本，默认为...
    /// 'break' 换行
    /// 'breakAll' 换行，跟'break'不同的是，在英语等拉丁文中，'breakAll'还会强制单词内换行
    ///
    overflow: Option<String>,
    ellipsis: Option<bool>
}