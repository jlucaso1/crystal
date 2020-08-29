/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryPlayerGameDelta {
    #[serde(rename = "champMastery", skip_serializing_if = "Option::is_none")]
    pub champ_mastery: Option<crate::models::LolMatchHistoryMatchHistoryPlayerChampMasteryDelta>,
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "gamePlatformId", skip_serializing_if = "Option::is_none")]
    pub game_platform_id: Option<String>,
    #[serde(rename = "leagueDelta", skip_serializing_if = "Option::is_none")]
    pub league_delta: Option<crate::models::LolMatchHistoryMatchHistoryPlayerLeagueDelta>,
    #[serde(rename = "platformDelta", skip_serializing_if = "Option::is_none")]
    pub platform_delta: Option<crate::models::LolMatchHistoryMatchHistoryPlayerPlatformDelta>,
}

impl LolMatchHistoryMatchHistoryPlayerGameDelta {
    pub fn new() -> LolMatchHistoryMatchHistoryPlayerGameDelta {
        LolMatchHistoryMatchHistoryPlayerGameDelta {
            champ_mastery: None,
            game_id: None,
            game_platform_id: None,
            league_delta: None,
            platform_delta: None,
        }
    }
}

