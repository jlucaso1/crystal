/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhaseRosterSubDto {
    #[serde(rename = "bet", skip_serializing_if = "Option::is_none")]
    pub bet: Option<i32>,
    #[serde(rename = "betType", skip_serializing_if = "Option::is_none")]
    pub bet_type: Option<crate::models::TicketType>,
    #[serde(rename = "pay", skip_serializing_if = "Option::is_none")]
    pub pay: Option<i32>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<crate::models::Position>,
    #[serde(rename = "premiumPay", skip_serializing_if = "Option::is_none")]
    pub premium_pay: Option<i32>,
    #[serde(rename = "replacedPlayerId", skip_serializing_if = "Option::is_none")]
    pub replaced_player_id: Option<i64>,
    #[serde(rename = "subState", skip_serializing_if = "Option::is_none")]
    pub sub_state: Option<crate::models::SubState>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<i32>,
}

impl PhaseRosterSubDto {
    pub fn new() -> PhaseRosterSubDto {
        PhaseRosterSubDto {
            bet: None,
            bet_type: None,
            pay: None,
            player_id: None,
            position: None,
            premium_pay: None,
            replaced_player_id: None,
            sub_state: None,
            tier: None,
        }
    }
}


