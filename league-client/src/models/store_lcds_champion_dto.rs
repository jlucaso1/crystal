/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoreLcdsChampionDto {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "botEnabled", skip_serializing_if = "Option::is_none")]
    pub bot_enabled: Option<bool>,
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "championSkins", skip_serializing_if = "Option::is_none")]
    pub champion_skins: Option<Vec<crate::models::StoreLcdsChampionSkinDto>>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
    #[serde(rename = "freeToPlay", skip_serializing_if = "Option::is_none")]
    pub free_to_play: Option<bool>,
    #[serde(rename = "freeToPlayReward", skip_serializing_if = "Option::is_none")]
    pub free_to_play_reward: Option<bool>,
    #[serde(rename = "owned", skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
    #[serde(rename = "purchaseDate", skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<i64>,
    #[serde(rename = "rankedPlayEnabled", skip_serializing_if = "Option::is_none")]
    pub ranked_play_enabled: Option<bool>,
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    #[serde(rename = "winCountRemaining", skip_serializing_if = "Option::is_none")]
    pub win_count_remaining: Option<i32>,
}

impl StoreLcdsChampionDto {
    pub fn new() -> StoreLcdsChampionDto {
        StoreLcdsChampionDto {
            active: None,
            bot_enabled: None,
            champion_id: None,
            champion_skins: None,
            end_date: None,
            free_to_play: None,
            free_to_play_reward: None,
            owned: None,
            purchase_date: None,
            ranked_play_enabled: None,
            sources: None,
            win_count_remaining: None,
        }
    }
}


