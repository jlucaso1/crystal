/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyPlayerCollectionDto {
    #[serde(rename = "players", skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<crate::models::LolLobbyPlayerDto>>,
}

impl LolLobbyPlayerCollectionDto {
    pub fn new() -> LolLobbyPlayerCollectionDto {
        LolLobbyPlayerCollectionDto {
            players: None,
        }
    }
}

