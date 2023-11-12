use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTemplate<NPData> {
    pub success: bool,
    pub data: Vec<NPData>,
    pub errors: Vec<String>,
    pub warnings: Vec<serde_json::Value>,
    pub info: serde_json::Value,
    pub message_codes: Vec<String>,
    pub error_codes: Vec<String>,
    pub warning_codes: Vec<String>,
    pub info_codes: Vec<serde_json::Value>,
}
