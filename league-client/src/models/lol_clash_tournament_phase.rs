/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolClashTournamentPhase {
    #[serde(rename = "cancelled", skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<bool>,
    #[serde(rename = "capacityStatus", skip_serializing_if = "Option::is_none")]
    pub capacity_status: Option<crate::models::CapacityEnum>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "limitTiers", skip_serializing_if = "Option::is_none")]
    pub limit_tiers: Option<Vec<i32>>,
    #[serde(rename = "lockinStartTime", skip_serializing_if = "Option::is_none")]
    pub lockin_start_time: Option<i64>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "scoutingStartTime", skip_serializing_if = "Option::is_none")]
    pub scouting_start_time: Option<i64>,
    #[serde(rename = "tournamentId", skip_serializing_if = "Option::is_none")]
    pub tournament_id: Option<i64>,
}

impl LolClashTournamentPhase {
    pub fn new() -> LolClashTournamentPhase {
        LolClashTournamentPhase {
            cancelled: None,
            capacity_status: None,
            id: None,
            limit_tiers: None,
            lockin_start_time: None,
            period: None,
            scouting_start_time: None,
            tournament_id: None,
        }
    }
}


