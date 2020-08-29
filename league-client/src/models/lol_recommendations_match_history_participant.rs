/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolRecommendationsMatchHistoryParticipant {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "highestAchievedSeasonTier", skip_serializing_if = "Option::is_none")]
    pub highest_achieved_season_tier: Option<String>,
    #[serde(rename = "participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<i32>,
    #[serde(rename = "spell1Id", skip_serializing_if = "Option::is_none")]
    pub spell1_id: Option<i32>,
    #[serde(rename = "spell2Id", skip_serializing_if = "Option::is_none")]
    pub spell2_id: Option<i32>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
}

impl LolRecommendationsMatchHistoryParticipant {
    pub fn new() -> LolRecommendationsMatchHistoryParticipant {
        LolRecommendationsMatchHistoryParticipant {
            champion_id: None,
            highest_achieved_season_tier: None,
            participant_id: None,
            spell1_id: None,
            spell2_id: None,
            team_id: None,
        }
    }
}


