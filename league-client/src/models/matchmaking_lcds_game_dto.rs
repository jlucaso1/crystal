/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchmakingLcdsGameDto {
    #[serde(rename = "gameState", skip_serializing_if = "Option::is_none")]
    pub game_state: Option<String>,
    #[serde(rename = "statusOfParticipants", skip_serializing_if = "Option::is_none")]
    pub status_of_participants: Option<String>,
    #[serde(rename = "teamOne", skip_serializing_if = "Option::is_none")]
    pub team_one: Option<Vec<crate::models::MatchmakingLcdsPlayerParticipant>>,
    #[serde(rename = "teamTwo", skip_serializing_if = "Option::is_none")]
    pub team_two: Option<Vec<crate::models::MatchmakingLcdsPlayerParticipant>>,
    #[serde(rename = "terminatedCondition", skip_serializing_if = "Option::is_none")]
    pub terminated_condition: Option<String>,
}

impl MatchmakingLcdsGameDto {
    pub fn new() -> MatchmakingLcdsGameDto {
        MatchmakingLcdsGameDto {
            game_state: None,
            status_of_participants: None,
            team_one: None,
            team_two: None,
            terminated_condition: None,
        }
    }
}


