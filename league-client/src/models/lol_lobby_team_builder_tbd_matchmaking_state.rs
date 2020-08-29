/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyTeamBuilderTbdMatchmakingState {
    #[serde(rename = "backwardsTransitionReason", skip_serializing_if = "Option::is_none")]
    pub backwards_transition_reason: Option<String>,
    #[serde(rename = "estimatedMatchmakingTimeMillis", skip_serializing_if = "Option::is_none")]
    pub estimated_matchmaking_time_millis: Option<i64>,
    #[serde(rename = "timeInMatchmakingMillis", skip_serializing_if = "Option::is_none")]
    pub time_in_matchmaking_millis: Option<i64>,
}

impl LolLobbyTeamBuilderTbdMatchmakingState {
    pub fn new() -> LolLobbyTeamBuilderTbdMatchmakingState {
        LolLobbyTeamBuilderTbdMatchmakingState {
            backwards_transition_reason: None,
            estimated_matchmaking_time_millis: None,
            time_in_matchmaking_millis: None,
        }
    }
}


