/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyGameflowGameDodge {
    #[serde(rename = "dodgeIds", skip_serializing_if = "Option::is_none")]
    pub dodge_ids: Option<Vec<i64>>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<crate::models::LolLobbyGameflowPhase>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolLobbyMatchmakingDodgeState>,
}

impl LolLobbyGameflowGameDodge {
    pub fn new() -> LolLobbyGameflowGameDodge {
        LolLobbyGameflowGameDodge {
            dodge_ids: None,
            phase: None,
            state: None,
        }
    }
}


