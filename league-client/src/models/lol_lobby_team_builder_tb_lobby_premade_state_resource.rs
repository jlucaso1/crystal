/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyTeamBuilderTbLobbyPremadeStateResource {
    #[serde(rename = "autoFillEligible", skip_serializing_if = "Option::is_none")]
    pub auto_fill_eligible: Option<bool>,
    #[serde(rename = "autoFillProtectedForPromos", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_promos: Option<bool>,
    #[serde(rename = "autoFillProtectedForStreaking", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_streaking: Option<bool>,
    #[serde(rename = "backwardsToPremadeTransitionReason", skip_serializing_if = "Option::is_none")]
    pub backwards_to_premade_transition_reason: Option<crate::models::LolLobbyTeamBuilderTbLobbyBackwardsToPremadeTransitionResource>,
    #[serde(rename = "captainSlotId", skip_serializing_if = "Option::is_none")]
    pub captain_slot_id: Option<i32>,
    #[serde(rename = "draftPremadeId", skip_serializing_if = "Option::is_none")]
    pub draft_premade_id: Option<String>,
    #[serde(rename = "draftSlots", skip_serializing_if = "Option::is_none")]
    pub draft_slots: Option<Vec<crate::models::LolLobbyTeamBuilderTbLobbySlotResource>>,
    #[serde(rename = "localPlayerSlotId", skip_serializing_if = "Option::is_none")]
    pub local_player_slot_id: Option<i32>,
    #[serde(rename = "playableDraftPositions", skip_serializing_if = "Option::is_none")]
    pub playable_draft_positions: Option<Vec<String>>,
    #[serde(rename = "premadeChatRoomId", skip_serializing_if = "Option::is_none")]
    pub premade_chat_room_id: Option<String>,
    #[serde(rename = "readyState", skip_serializing_if = "Option::is_none")]
    pub ready_state: Option<crate::models::LolLobbyTeamBuilderReadyStateV1>,
    #[serde(rename = "readyToMatchmake", skip_serializing_if = "Option::is_none")]
    pub ready_to_matchmake: Option<bool>,
    #[serde(rename = "showPositionExcluder", skip_serializing_if = "Option::is_none")]
    pub show_position_excluder: Option<bool>,
    #[serde(rename = "showPositionSelector", skip_serializing_if = "Option::is_none")]
    pub show_position_selector: Option<bool>,
    #[serde(rename = "timer", skip_serializing_if = "Option::is_none")]
    pub timer: Option<i64>,
}

impl LolLobbyTeamBuilderTbLobbyPremadeStateResource {
    pub fn new() -> LolLobbyTeamBuilderTbLobbyPremadeStateResource {
        LolLobbyTeamBuilderTbLobbyPremadeStateResource {
            auto_fill_eligible: None,
            auto_fill_protected_for_promos: None,
            auto_fill_protected_for_streaking: None,
            backwards_to_premade_transition_reason: None,
            captain_slot_id: None,
            draft_premade_id: None,
            draft_slots: None,
            local_player_slot_id: None,
            playable_draft_positions: None,
            premade_chat_room_id: None,
            ready_state: None,
            ready_to_matchmake: None,
            show_position_excluder: None,
            show_position_selector: None,
            timer: None,
        }
    }
}


