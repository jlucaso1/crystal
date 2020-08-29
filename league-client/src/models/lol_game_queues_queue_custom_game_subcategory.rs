/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolGameQueuesQueueCustomGameSubcategory {
    #[serde(rename = "customSpectatorPolicies", skip_serializing_if = "Option::is_none")]
    pub custom_spectator_policies: Option<Vec<crate::models::LolGameQueuesQueueCustomGameSpectatorPolicy>>,
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    #[serde(rename = "mapId", skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
    #[serde(rename = "maxLevel", skip_serializing_if = "Option::is_none")]
    pub max_level: Option<i32>,
    #[serde(rename = "maxPlayerCount", skip_serializing_if = "Option::is_none")]
    pub max_player_count: Option<i32>,
    #[serde(rename = "maximumParticipantListSize", skip_serializing_if = "Option::is_none")]
    pub maximum_participant_list_size: Option<i32>,
    #[serde(rename = "minLevel", skip_serializing_if = "Option::is_none")]
    pub min_level: Option<i32>,
    #[serde(rename = "minimumParticipantListSize", skip_serializing_if = "Option::is_none")]
    pub minimum_participant_list_size: Option<i32>,
    #[serde(rename = "mutators", skip_serializing_if = "Option::is_none")]
    pub mutators: Option<Vec<crate::models::LolGameQueuesQueueGameTypeConfig>>,
    #[serde(rename = "numPlayersPerTeam", skip_serializing_if = "Option::is_none")]
    pub num_players_per_team: Option<i32>,
    #[serde(rename = "queueAvailability", skip_serializing_if = "Option::is_none")]
    pub queue_availability: Option<crate::models::LolGameQueuesQueueAvailability>,
}

impl LolGameQueuesQueueCustomGameSubcategory {
    pub fn new() -> LolGameQueuesQueueCustomGameSubcategory {
        LolGameQueuesQueueCustomGameSubcategory {
            custom_spectator_policies: None,
            game_mode: None,
            map_id: None,
            max_level: None,
            max_player_count: None,
            maximum_participant_list_size: None,
            min_level: None,
            minimum_participant_list_size: None,
            mutators: None,
            num_players_per_team: None,
            queue_availability: None,
        }
    }
}

