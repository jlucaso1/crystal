/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyPartyDto {
    #[serde(rename = "activeRestrictions", skip_serializing_if = "Option::is_none")]
    pub active_restrictions: Option<crate::models::LolLobbyQueueRestrictionDto>,
    #[serde(rename = "activityLocked", skip_serializing_if = "Option::is_none")]
    pub activity_locked: Option<bool>,
    #[serde(rename = "activityResumeUtcMillis", skip_serializing_if = "Option::is_none")]
    pub activity_resume_utc_millis: Option<i64>,
    #[serde(rename = "activityStartedUtcMillis", skip_serializing_if = "Option::is_none")]
    pub activity_started_utc_millis: Option<i64>,
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<crate::models::LolLobbyPartyChatDto>,
    #[serde(rename = "eligibilityHash", skip_serializing_if = "Option::is_none")]
    pub eligibility_hash: Option<i64>,
    #[serde(rename = "eligibilityRestrictions", skip_serializing_if = "Option::is_none")]
    pub eligibility_restrictions: Option<Vec<crate::models::LolLobbyGatekeeperRestrictionDto>>,
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<crate::models::LolLobbyGameModeDto>,
    #[serde(rename = "maxPartySize", skip_serializing_if = "Option::is_none")]
    pub max_party_size: Option<i32>,
    #[serde(rename = "partyId", skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    #[serde(rename = "partyType", skip_serializing_if = "Option::is_none")]
    pub party_type: Option<String>,
    #[serde(rename = "platformId", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "players", skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<crate::models::LolLobbyPartyMemberDto>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl LolLobbyPartyDto {
    pub fn new() -> LolLobbyPartyDto {
        LolLobbyPartyDto {
            active_restrictions: None,
            activity_locked: None,
            activity_resume_utc_millis: None,
            activity_started_utc_millis: None,
            chat: None,
            eligibility_hash: None,
            eligibility_restrictions: None,
            game_mode: None,
            max_party_size: None,
            party_id: None,
            party_type: None,
            platform_id: None,
            players: None,
            version: None,
        }
    }
}

