use serde::{Deserialize, Serialize};
///
///
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Truncate{

    ///
    /// 截断文本的最大长度，超过此长度会被截断。
    ///
    max_width:f32,

    ///
    /// 截断后文字末尾显示的内容。
    ///
    ellipsis:Option<String>,
}