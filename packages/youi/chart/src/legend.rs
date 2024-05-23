use serde::{Serialize,Deserialize};
use crate::style::TextStyle;

///
/// 图例组件。
/// 图例组件展现了不同系列的标记(symbol)，颜色和名字。可以通过点击图例控制哪些系列不显示。
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legend{
    #[serde(skip_serializing_if = "Option::is_none")]
    data:Option<Vec<LegendData>>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegendData{

    String(String),

    DataItem(DataItem)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataItem{
    ///
    ///
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    name:Option<String>,
    ///
    ///
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    icon:Option<String>,

    ///
    ///
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    text_style:Option<TextStyle>,

    //tooltip?: unknown;

}
