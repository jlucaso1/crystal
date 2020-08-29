/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyRankedStatsUnsignedDto {
    #[serde(rename = "highestPreviousSeasonEndRank", skip_serializing_if = "Option::is_none")]
    pub highest_previous_season_end_rank: Option<String>,
    #[serde(rename = "highestPreviousSeasonEndTier", skip_serializing_if = "Option::is_none")]
    pub highest_previous_season_end_tier: Option<String>,
    #[serde(rename = "jwt", skip_serializing_if = "Option::is_none")]
    pub jwt: Option<String>,
    #[serde(rename = "queues", skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<crate::models::LolLobbyRankedPositionInfoDto>>,
}

impl LolLobbyRankedStatsUnsignedDto {
    pub fn new() -> LolLobbyRankedStatsUnsignedDto {
        LolLobbyRankedStatsUnsignedDto {
            highest_previous_season_end_rank: None,
            highest_previous_season_end_tier: None,
            jwt: None,
            queues: None,
        }
    }
}


