/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatGameflowGameData {
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "playerChampionSelections", skip_serializing_if = "Option::is_none")]
    pub player_champion_selections: Option<Vec<crate::models::LolChatChampSelection>>,
    #[serde(rename = "queue", skip_serializing_if = "Option::is_none")]
    pub queue: Option<crate::models::LolChatQueue>,
    #[serde(rename = "teamOne", skip_serializing_if = "Option::is_none")]
    pub team_one: Option<Vec<crate::models::LolChatTeamPlayerEntry>>,
    #[serde(rename = "teamTwo", skip_serializing_if = "Option::is_none")]
    pub team_two: Option<Vec<crate::models::LolChatTeamPlayerEntry>>,
}

impl LolChatGameflowGameData {
    pub fn new() -> LolChatGameflowGameData {
        LolChatGameflowGameData {
            game_id: None,
            player_champion_selections: None,
            queue: None,
            team_one: None,
            team_two: None,
        }
    }
}


