/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPatchChunkingPatcherEnvironment {
    #[serde(rename = "client_patcher_available", skip_serializing_if = "Option::is_none")]
    pub client_patcher_available: Option<bool>,
    #[serde(rename = "client_patcher_enabled", skip_serializing_if = "Option::is_none")]
    pub client_patcher_enabled: Option<bool>,
    #[serde(rename = "game_patcher_available", skip_serializing_if = "Option::is_none")]
    pub game_patcher_available: Option<bool>,
    #[serde(rename = "game_patcher_enabled", skip_serializing_if = "Option::is_none")]
    pub game_patcher_enabled: Option<bool>,
}

impl LolPatchChunkingPatcherEnvironment {
    pub fn new() -> LolPatchChunkingPatcherEnvironment {
        LolPatchChunkingPatcherEnvironment {
            client_patcher_available: None,
            client_patcher_enabled: None,
            game_patcher_available: None,
            game_patcher_enabled: None,
        }
    }
}


