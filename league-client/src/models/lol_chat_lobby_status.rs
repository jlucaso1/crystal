/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatLobbyStatus {
    #[serde(rename = "customSpectatorPolicy", skip_serializing_if = "Option::is_none")]
    pub custom_spectator_policy: Option<crate::models::LolChatQueueCustomGameSpectatorPolicy>,
    #[serde(rename = "isCustom", skip_serializing_if = "Option::is_none")]
    pub is_custom: Option<bool>,
    #[serde(rename = "isLeader", skip_serializing_if = "Option::is_none")]
    pub is_leader: Option<bool>,
    #[serde(rename = "isPracticeTool", skip_serializing_if = "Option::is_none")]
    pub is_practice_tool: Option<bool>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
}

impl LolChatLobbyStatus {
    pub fn new() -> LolChatLobbyStatus {
        LolChatLobbyStatus {
            custom_spectator_policy: None,
            is_custom: None,
            is_leader: None,
            is_practice_tool: None,
            queue_id: None,
        }
    }
}

