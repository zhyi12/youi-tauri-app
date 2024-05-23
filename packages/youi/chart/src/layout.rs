use crate::OrdinalRawValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoxLayout{
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<OrdinalRawValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<OrdinalRawValue>,
}
