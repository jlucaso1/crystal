/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolNpeRewardsReward {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "renderer", skip_serializing_if = "Option::is_none")]
    pub renderer: Option<String>,
}

impl LolNpeRewardsReward {
    pub fn new() -> LolNpeRewardsReward {
        LolNpeRewardsReward {
            data: None,
            renderer: None,
        }
    }
}


