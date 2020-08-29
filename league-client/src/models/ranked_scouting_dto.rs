/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RankedScoutingDto {
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
    #[serde(rename = "topMasteries", skip_serializing_if = "Option::is_none")]
    pub top_masteries: Option<Vec<crate::models::ChampionMasteryPublicDto>>,
    #[serde(rename = "topSeasonChampions", skip_serializing_if = "Option::is_none")]
    pub top_season_champions: Option<Vec<crate::models::ChampionScoutingDto>>,
    #[serde(rename = "totalMasteryScore", skip_serializing_if = "Option::is_none")]
    pub total_mastery_score: Option<i64>,
}

impl RankedScoutingDto {
    pub fn new() -> RankedScoutingDto {
        RankedScoutingDto {
            player_id: None,
            top_masteries: None,
            top_season_champions: None,
            total_mastery_score: None,
        }
    }
}

