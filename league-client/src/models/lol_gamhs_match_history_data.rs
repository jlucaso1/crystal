/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolGamhsMatchHistoryData {
    #[serde(rename = "json", skip_serializing_if = "Option::is_none")]
    pub json: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::LolGamhsMatchHistoryMetadata>,
}

impl LolGamhsMatchHistoryData {
    pub fn new() -> LolGamhsMatchHistoryData {
        LolGamhsMatchHistoryData {
            json: None,
            metadata: None,
        }
    }
}

