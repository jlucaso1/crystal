/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyTeamBuilderTbLobbySlotResource {
    #[serde(rename = "autoFillEligible", skip_serializing_if = "Option::is_none")]
    pub auto_fill_eligible: Option<bool>,
    #[serde(rename = "autoFillProtectedForPromos", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_promos: Option<bool>,
    #[serde(rename = "autoFillProtectedForSoloing", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_soloing: Option<bool>,
    #[serde(rename = "autoFillProtectedForStreaking", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_streaking: Option<bool>,
    #[serde(rename = "draftPositionPreferences", skip_serializing_if = "Option::is_none")]
    pub draft_position_preferences: Option<Vec<String>>,
    #[serde(rename = "excludedPositionPreference", skip_serializing_if = "Option::is_none")]
    pub excluded_position_preference: Option<String>,
    #[serde(rename = "showPositionExcluder", skip_serializing_if = "Option::is_none")]
    pub show_position_excluder: Option<bool>,
    #[serde(rename = "slotId", skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<i32>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolLobbyTeamBuilderTbLobbySlotResource {
    pub fn new() -> LolLobbyTeamBuilderTbLobbySlotResource {
        LolLobbyTeamBuilderTbLobbySlotResource {
            auto_fill_eligible: None,
            auto_fill_protected_for_promos: None,
            auto_fill_protected_for_soloing: None,
            auto_fill_protected_for_streaking: None,
            draft_position_preferences: None,
            excluded_position_preference: None,
            show_position_excluder: None,
            slot_id: None,
            summoner_id: None,
        }
    }
}

