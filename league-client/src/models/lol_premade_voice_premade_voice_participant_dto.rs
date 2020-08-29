/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPremadeVoicePremadeVoiceParticipantDto {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "energy", skip_serializing_if = "Option::is_none")]
    pub energy: Option<i32>,
    #[serde(rename = "isMuted", skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,
    #[serde(rename = "isSpeaking", skip_serializing_if = "Option::is_none")]
    pub is_speaking: Option<bool>,
    #[serde(rename = "participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<i32>,
}

impl LolPremadeVoicePremadeVoiceParticipantDto {
    pub fn new() -> LolPremadeVoicePremadeVoiceParticipantDto {
        LolPremadeVoicePremadeVoiceParticipantDto {
            display_name: None,
            energy: None,
            is_muted: None,
            is_speaking: None,
            participant_id: None,
            puuid: None,
            summoner_id: None,
            volume: None,
        }
    }
}

