/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyTeamBuilderTbLobbyBackwardsToPremadeTransitionResource {
    #[serde(rename = "backwardsTransitionReason", skip_serializing_if = "Option::is_none")]
    pub backwards_transition_reason: Option<String>,
    #[serde(rename = "slotIds", skip_serializing_if = "Option::is_none")]
    pub slot_ids: Option<Vec<i32>>,
}

impl LolLobbyTeamBuilderTbLobbyBackwardsToPremadeTransitionResource {
    pub fn new() -> LolLobbyTeamBuilderTbLobbyBackwardsToPremadeTransitionResource {
        LolLobbyTeamBuilderTbLobbyBackwardsToPremadeTransitionResource {
            backwards_transition_reason: None,
            slot_ids: None,
        }
    }
}


