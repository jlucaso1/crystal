/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolMatchmakingMatchmakingDodgeData {
    #[serde(rename = "dodgerId", skip_serializing_if = "Option::is_none")]
    pub dodger_id: Option<i64>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolMatchmakingMatchmakingDodgeState>,
}

impl LolMatchmakingMatchmakingDodgeData {
    pub fn new() -> LolMatchmakingMatchmakingDodgeData {
        LolMatchmakingMatchmakingDodgeData {
            dodger_id: None,
            state: None,
        }
    }
}

