use serde::{Serialize,Deserialize};
use crate::component::Component;
use crate::layout::BoxLayout;
use crate::style::Shadow;

///
/// 
/// 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grid{

    #[serde(flatten)]
    component:Component,

    #[serde(flatten)]
    layout:BoxLayout,
    
    #[serde(flatten)]
    shadow:Shadow,
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contain_label: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    // tooltip: any;
}