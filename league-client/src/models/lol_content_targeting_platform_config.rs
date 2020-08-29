/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolContentTargetingPlatformConfig {
    #[serde(rename = "ABTestFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub ab_test_filter_enabled: Option<bool>,
    #[serde(rename = "ABTestFilterGroups", skip_serializing_if = "Option::is_none")]
    pub ab_test_filter_groups: Option<i64>,
    #[serde(rename = "ABTestFilterSalt", skip_serializing_if = "Option::is_none")]
    pub ab_test_filter_salt: Option<i64>,
    #[serde(rename = "AsynchronousEventHandlerSetupDelayInSeconds", skip_serializing_if = "Option::is_none")]
    pub asynchronous_event_handler_setup_delay_in_seconds: Option<i32>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EntitlementsFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub entitlements_filter_enabled: Option<bool>,
    #[serde(rename = "EntitlementsPrefix", skip_serializing_if = "Option::is_none")]
    pub entitlements_prefix: Option<String>,
    #[serde(rename = "InactiveFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub inactive_filter_enabled: Option<bool>,
    #[serde(rename = "LaneFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub lane_filter_enabled: Option<bool>,
    #[serde(rename = "LaneFilterThreshold", skip_serializing_if = "Option::is_none")]
    pub lane_filter_threshold: Option<i32>,
    #[serde(rename = "LevelFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub level_filter_enabled: Option<bool>,
    #[serde(rename = "LocationFiltersEnabled", skip_serializing_if = "Option::is_none")]
    pub location_filters_enabled: Option<bool>,
    #[serde(rename = "MainFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub main_filter_enabled: Option<bool>,
    #[serde(rename = "Mapping", skip_serializing_if = "Option::is_none")]
    pub mapping: Option<String>,
    #[serde(rename = "MasteryFilterChampionLimit", skip_serializing_if = "Option::is_none")]
    pub mastery_filter_champion_limit: Option<i32>,
    #[serde(rename = "MasteryFilterDaysSinceLastPlayed", skip_serializing_if = "Option::is_none")]
    pub mastery_filter_days_since_last_played: Option<i32>,
    #[serde(rename = "MasteryFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub mastery_filter_enabled: Option<bool>,
    #[serde(rename = "MasteryFilterLevelThreshold", skip_serializing_if = "Option::is_none")]
    pub mastery_filter_level_threshold: Option<i32>,
    #[serde(rename = "MissionsFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub missions_filter_enabled: Option<bool>,
    #[serde(rename = "RankFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub rank_filter_enabled: Option<bool>,
    #[serde(rename = "RankedFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub ranked_filter_enabled: Option<bool>,
    #[serde(rename = "SummonerIconFilterEnabled", skip_serializing_if = "Option::is_none")]
    pub summoner_icon_filter_enabled: Option<bool>,
    #[serde(rename = "TargetingAttributeStorageBaseUri", skip_serializing_if = "Option::is_none")]
    pub targeting_attribute_storage_base_uri: Option<String>,
    #[serde(rename = "TargetingAttributeStorageEnabled", skip_serializing_if = "Option::is_none")]
    pub targeting_attribute_storage_enabled: Option<bool>,
    #[serde(rename = "TasIngestionDelayInSeconds", skip_serializing_if = "Option::is_none")]
    pub tas_ingestion_delay_in_seconds: Option<i32>,
}

impl LolContentTargetingPlatformConfig {
    pub fn new() -> LolContentTargetingPlatformConfig {
        LolContentTargetingPlatformConfig {
            ab_test_filter_enabled: None,
            ab_test_filter_groups: None,
            ab_test_filter_salt: None,
            asynchronous_event_handler_setup_delay_in_seconds: None,
            enabled: None,
            entitlements_filter_enabled: None,
            entitlements_prefix: None,
            inactive_filter_enabled: None,
            lane_filter_enabled: None,
            lane_filter_threshold: None,
            level_filter_enabled: None,
            location_filters_enabled: None,
            main_filter_enabled: None,
            mapping: None,
            mastery_filter_champion_limit: None,
            mastery_filter_days_since_last_played: None,
            mastery_filter_enabled: None,
            mastery_filter_level_threshold: None,
            missions_filter_enabled: None,
            rank_filter_enabled: None,
            ranked_filter_enabled: None,
            summoner_icon_filter_enabled: None,
            targeting_attribute_storage_base_uri: None,
            targeting_attribute_storage_enabled: None,
            tas_ingestion_delay_in_seconds: None,
        }
    }
}


