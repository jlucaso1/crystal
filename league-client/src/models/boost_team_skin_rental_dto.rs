/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoostTeamSkinRentalDto {
    #[serde(rename = "availableSkins", skip_serializing_if = "Option::is_none")]
    pub available_skins: Option<Vec<i64>>,
    #[serde(rename = "ipReward", skip_serializing_if = "Option::is_none")]
    pub ip_reward: Option<i64>,
    #[serde(rename = "ipRewardForPurchaser", skip_serializing_if = "Option::is_none")]
    pub ip_reward_for_purchaser: Option<i64>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(rename = "skinUnlockMode", skip_serializing_if = "Option::is_none")]
    pub skin_unlock_mode: Option<String>,
    #[serde(rename = "summonerName", skip_serializing_if = "Option::is_none")]
    pub summoner_name: Option<String>,
    #[serde(rename = "unlocked", skip_serializing_if = "Option::is_none")]
    pub unlocked: Option<bool>,
}

impl BoostTeamSkinRentalDto {
    pub fn new() -> BoostTeamSkinRentalDto {
        BoostTeamSkinRentalDto {
            available_skins: None,
            ip_reward: None,
            ip_reward_for_purchaser: None,
            price: None,
            skin_unlock_mode: None,
            summoner_name: None,
            unlocked: None,
        }
    }
}


