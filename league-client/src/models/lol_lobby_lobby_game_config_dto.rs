/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyLobbyGameConfigDto {
    #[serde(rename = "allowablePremadeSizes", skip_serializing_if = "Option::is_none")]
    pub allowable_premade_sizes: Option<Vec<i32>>,
    #[serde(rename = "customLobbyName", skip_serializing_if = "Option::is_none")]
    pub custom_lobby_name: Option<String>,
    #[serde(rename = "customMutatorName", skip_serializing_if = "Option::is_none")]
    pub custom_mutator_name: Option<String>,
    #[serde(rename = "customRewardsDisabledReasons", skip_serializing_if = "Option::is_none")]
    pub custom_rewards_disabled_reasons: Option<Vec<String>>,
    #[serde(rename = "customSpectatorPolicy", skip_serializing_if = "Option::is_none")]
    pub custom_spectator_policy: Option<crate::models::LolLobbyQueueCustomGameSpectatorPolicy>,
    #[serde(rename = "customSpectators", skip_serializing_if = "Option::is_none")]
    pub custom_spectators: Option<Vec<crate::models::LolLobbyLobbyParticipantDto>>,
    #[serde(rename = "customTeam100", skip_serializing_if = "Option::is_none")]
    pub custom_team100: Option<Vec<crate::models::LolLobbyLobbyParticipantDto>>,
    #[serde(rename = "customTeam200", skip_serializing_if = "Option::is_none")]
    pub custom_team200: Option<Vec<crate::models::LolLobbyLobbyParticipantDto>>,
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    #[serde(rename = "isCustom", skip_serializing_if = "Option::is_none")]
    pub is_custom: Option<bool>,
    #[serde(rename = "isLobbyFull", skip_serializing_if = "Option::is_none")]
    pub is_lobby_full: Option<bool>,
    #[serde(rename = "isTeamBuilderManaged", skip_serializing_if = "Option::is_none")]
    pub is_team_builder_managed: Option<bool>,
    #[serde(rename = "mapId", skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
    #[serde(rename = "maxHumanPlayers", skip_serializing_if = "Option::is_none")]
    pub max_human_players: Option<i32>,
    #[serde(rename = "maxLobbySize", skip_serializing_if = "Option::is_none")]
    pub max_lobby_size: Option<i32>,
    #[serde(rename = "maxTeamSize", skip_serializing_if = "Option::is_none")]
    pub max_team_size: Option<i32>,
    #[serde(rename = "pickType", skip_serializing_if = "Option::is_none")]
    pub pick_type: Option<String>,
    #[serde(rename = "premadeSizeAllowed", skip_serializing_if = "Option::is_none")]
    pub premade_size_allowed: Option<bool>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[serde(rename = "showPositionSelector", skip_serializing_if = "Option::is_none")]
    pub show_position_selector: Option<bool>,
}

impl LolLobbyLobbyGameConfigDto {
    pub fn new() -> LolLobbyLobbyGameConfigDto {
        LolLobbyLobbyGameConfigDto {
            allowable_premade_sizes: None,
            custom_lobby_name: None,
            custom_mutator_name: None,
            custom_rewards_disabled_reasons: None,
            custom_spectator_policy: None,
            custom_spectators: None,
            custom_team100: None,
            custom_team200: None,
            game_mode: None,
            is_custom: None,
            is_lobby_full: None,
            is_team_builder_managed: None,
            map_id: None,
            max_human_players: None,
            max_lobby_size: None,
            max_team_size: None,
            pick_type: None,
            premade_size_allowed: None,
            queue_id: None,
            show_position_selector: None,
        }
    }
}

