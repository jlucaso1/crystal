/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyLcdsGameMetaData {
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "mapId", skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
}

impl LolLobbyLcdsGameMetaData {
    pub fn new() -> LolLobbyLcdsGameMetaData {
        LolLobbyLcdsGameMetaData {
            game_id: None,
            map_id: None,
        }
    }
}


