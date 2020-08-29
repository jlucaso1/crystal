/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolSummonerXpAndLevelMessage {
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<crate::models::LolSummonerLevelField>,
    #[serde(rename = "xp", skip_serializing_if = "Option::is_none")]
    pub xp: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl LolSummonerXpAndLevelMessage {
    pub fn new() -> LolSummonerXpAndLevelMessage {
        LolSummonerXpAndLevelMessage {
            level: None,
            xp: None,
        }
    }
}


