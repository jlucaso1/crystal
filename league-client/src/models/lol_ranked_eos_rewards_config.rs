/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolRankedEosRewardsConfig {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<crate::models::LolRankedEosRewardsConfigEntry>>,
}

impl LolRankedEosRewardsConfig {
    pub fn new() -> LolRankedEosRewardsConfig {
        LolRankedEosRewardsConfig {
            config: None,
        }
    }
}


