/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolRegaliaSummonerProfileUpdate {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LolRegaliaSummonerProfileUpdate {
    pub fn new() -> LolRegaliaSummonerProfileUpdate {
        LolRegaliaSummonerProfileUpdate {
            key: None,
            value: None,
        }
    }
}

