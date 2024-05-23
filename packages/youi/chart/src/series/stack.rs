
use serde::{Serialize,Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stack{

    #[serde(skip_serializing_if = "Option::is_none")]
    stack:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stack_strategy:Option<String>,
}