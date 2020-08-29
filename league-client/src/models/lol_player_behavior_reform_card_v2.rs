/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPlayerBehaviorReformCardV2 {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "playerFacingMessage", skip_serializing_if = "Option::is_none")]
    pub player_facing_message: Option<String>,
    #[serde(rename = "punishedForGameIds", skip_serializing_if = "Option::is_none")]
    pub punished_for_game_ids: Option<Vec<i64>>,
    #[serde(rename = "punishedForReformCardChatLogs", skip_serializing_if = "Option::is_none")]
    pub punished_for_reform_card_chat_logs: Option<Vec<crate::models::LolPlayerBehaviorReformCardChatLogs>>,
    #[serde(rename = "punishedUntilDateMillis", skip_serializing_if = "Option::is_none")]
    pub punished_until_date_millis: Option<i64>,
    #[serde(rename = "punishmentLengthGames", skip_serializing_if = "Option::is_none")]
    pub punishment_length_games: Option<i64>,
    #[serde(rename = "punishmentLengthMillis", skip_serializing_if = "Option::is_none")]
    pub punishment_length_millis: Option<i64>,
    #[serde(rename = "punishmentReason", skip_serializing_if = "Option::is_none")]
    pub punishment_reason: Option<String>,
    #[serde(rename = "punishmentType", skip_serializing_if = "Option::is_none")]
    pub punishment_type: Option<String>,
}

impl LolPlayerBehaviorReformCardV2 {
    pub fn new() -> LolPlayerBehaviorReformCardV2 {
        LolPlayerBehaviorReformCardV2 {
            id: None,
            player_facing_message: None,
            punished_for_game_ids: None,
            punished_for_reform_card_chat_logs: None,
            punished_until_date_millis: None,
            punishment_length_games: None,
            punishment_length_millis: None,
            punishment_reason: None,
            punishment_type: None,
        }
    }
}


