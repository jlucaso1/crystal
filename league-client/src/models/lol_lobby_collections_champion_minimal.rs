/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyCollectionsChampionMinimal {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "disabledQueues", skip_serializing_if = "Option::is_none")]
    pub disabled_queues: Option<Vec<String>>,
    #[serde(rename = "freeToPlay", skip_serializing_if = "Option::is_none")]
    pub free_to_play: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<crate::models::LolLobbyCollectionsOwnership>,
}

impl LolLobbyCollectionsChampionMinimal {
    pub fn new() -> LolLobbyCollectionsChampionMinimal {
        LolLobbyCollectionsChampionMinimal {
            active: None,
            disabled_queues: None,
            free_to_play: None,
            id: None,
            ownership: None,
        }
    }
}

