/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolMatchmakingGameflowQueue {
    #[serde(rename = "gameTypeConfig", skip_serializing_if = "Option::is_none")]
    pub game_type_config: Option<crate::models::LolMatchmakingGameflowGameTypeConfig>,
}

impl LolMatchmakingGameflowQueue {
    pub fn new() -> LolMatchmakingGameflowQueue {
        LolMatchmakingGameflowQueue {
            game_type_config: None,
        }
    }
}

