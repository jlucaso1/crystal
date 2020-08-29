/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SanitizerSanitizeRequest {
    #[serde(rename = "aggressiveScan", skip_serializing_if = "Option::is_none")]
    pub aggressive_scan: Option<bool>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<Vec<String>>,
}

impl SanitizerSanitizeRequest {
    pub fn new() -> SanitizerSanitizeRequest {
        SanitizerSanitizeRequest {
            aggressive_scan: None,
            level: None,
            text: None,
            texts: None,
        }
    }
}

