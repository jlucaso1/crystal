/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LcdsLoyaltyRewards {
    #[serde(rename = "champions", skip_serializing_if = "Option::is_none")]
    pub champions: Option<Vec<crate::models::LcdsChampionReward>>,
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<crate::models::LcdsGlobalRewards>,
    #[serde(rename = "ipBoost", skip_serializing_if = "Option::is_none")]
    pub ip_boost: Option<i32>,
    #[serde(rename = "xpBoost", skip_serializing_if = "Option::is_none")]
    pub xp_boost: Option<i32>,
}

impl LcdsLoyaltyRewards {
    pub fn new() -> LcdsLoyaltyRewards {
        LcdsLoyaltyRewards {
            champions: None,
            global: None,
            ip_boost: None,
            xp_boost: None,
        }
    }
}

