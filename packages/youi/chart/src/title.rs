use serde::{Deserialize, Serialize};
///
/// 标题组件，包含主标题和副标题。
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title{
    #[serde(skip_serializing_if = "Option::is_none")]
    text:Option<String>
}