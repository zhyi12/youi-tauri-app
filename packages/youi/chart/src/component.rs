
use serde::{Serialize,Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component{

    #[serde(skip_serializing_if = "Option::is_none")]
    main_type:Option<String>,

    #[serde(rename = "type",skip_serializing_if = "Option::is_none")]
    component_type:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name:Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z:Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel:Option<usize>,


}
