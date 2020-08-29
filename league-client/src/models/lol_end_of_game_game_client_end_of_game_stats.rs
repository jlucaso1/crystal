/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolEndOfGameGameClientEndOfGameStats {
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    #[serde(rename = "isRanked", skip_serializing_if = "Option::is_none")]
    pub is_ranked: Option<bool>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[serde(rename = "statsBlock", skip_serializing_if = "Option::is_none")]
    pub stats_block: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl LolEndOfGameGameClientEndOfGameStats {
    pub fn new() -> LolEndOfGameGameClientEndOfGameStats {
        LolEndOfGameGameClientEndOfGameStats {
            game_id: None,
            game_mode: None,
            is_ranked: None,
            queue_id: None,
            stats_block: None,
        }
    }
}


