/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatRankedStats {
    #[serde(rename = "highestPreviousSeasonAchievedDivision", skip_serializing_if = "Option::is_none")]
    pub highest_previous_season_achieved_division: Option<crate::models::LolChatLeagueDivision>,
    #[serde(rename = "highestPreviousSeasonAchievedTier", skip_serializing_if = "Option::is_none")]
    pub highest_previous_season_achieved_tier: Option<crate::models::LolChatLeagueTier>,
    #[serde(rename = "highestRankedEntry", skip_serializing_if = "Option::is_none")]
    pub highest_ranked_entry: Option<crate::models::LolChatRankedQueueStats>,
    #[serde(rename = "rankedRegaliaLevel", skip_serializing_if = "Option::is_none")]
    pub ranked_regalia_level: Option<i32>,
}

impl LolChatRankedStats {
    pub fn new() -> LolChatRankedStats {
        LolChatRankedStats {
            highest_previous_season_achieved_division: None,
            highest_previous_season_achieved_tier: None,
            highest_ranked_entry: None,
            ranked_regalia_level: None,
        }
    }
}


