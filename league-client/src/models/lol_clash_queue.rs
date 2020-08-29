/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolClashQueue {
    #[serde(rename = "areFreeChampionsAllowed", skip_serializing_if = "Option::is_none")]
    pub are_free_champions_allowed: Option<bool>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<crate::models::LolClashQueueGameCategory>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "detailedDescription", skip_serializing_if = "Option::is_none")]
    pub detailed_description: Option<String>,
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    #[serde(rename = "gameTypeConfig", skip_serializing_if = "Option::is_none")]
    pub game_type_config: Option<crate::models::LolClashQueueGameTypeConfig>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isRanked", skip_serializing_if = "Option::is_none")]
    pub is_ranked: Option<bool>,
    #[serde(rename = "isTeamBuilderManaged", skip_serializing_if = "Option::is_none")]
    pub is_team_builder_managed: Option<bool>,
    #[serde(rename = "isTeamOnly", skip_serializing_if = "Option::is_none")]
    pub is_team_only: Option<bool>,
    #[serde(rename = "mapId", skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
    #[serde(rename = "maxLevel", skip_serializing_if = "Option::is_none")]
    pub max_level: Option<i32>,
    #[serde(rename = "maxSummonerLevelForFirstWinOfTheDay", skip_serializing_if = "Option::is_none")]
    pub max_summoner_level_for_first_win_of_the_day: Option<i32>,
    #[serde(rename = "maximumParticipantListSize", skip_serializing_if = "Option::is_none")]
    pub maximum_participant_list_size: Option<i32>,
    #[serde(rename = "minLevel", skip_serializing_if = "Option::is_none")]
    pub min_level: Option<i32>,
    #[serde(rename = "minimumParticipantListSize", skip_serializing_if = "Option::is_none")]
    pub minimum_participant_list_size: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numPlayersPerTeam", skip_serializing_if = "Option::is_none")]
    pub num_players_per_team: Option<i32>,
    #[serde(rename = "queueAvailability", skip_serializing_if = "Option::is_none")]
    pub queue_availability: Option<crate::models::LolClashQueueAvailability>,
    #[serde(rename = "queueRewards", skip_serializing_if = "Option::is_none")]
    pub queue_rewards: Option<crate::models::LolClashQueueReward>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "spectatorEnabled", skip_serializing_if = "Option::is_none")]
    pub spectator_enabled: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LolClashQueue {
    pub fn new() -> LolClashQueue {
        LolClashQueue {
            are_free_champions_allowed: None,
            category: None,
            description: None,
            detailed_description: None,
            game_mode: None,
            game_type_config: None,
            id: None,
            is_ranked: None,
            is_team_builder_managed: None,
            is_team_only: None,
            map_id: None,
            max_level: None,
            max_summoner_level_for_first_win_of_the_day: None,
            maximum_participant_list_size: None,
            min_level: None,
            minimum_participant_list_size: None,
            name: None,
            num_players_per_team: None,
            queue_availability: None,
            queue_rewards: None,
            short_name: None,
            spectator_enabled: None,
            _type: None,
        }
    }
}


