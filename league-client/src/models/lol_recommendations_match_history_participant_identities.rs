/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolRecommendationsMatchHistoryParticipantIdentities {
    #[serde(rename = "participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<i32>,
    #[serde(rename = "player", skip_serializing_if = "Option::is_none")]
    pub player: Option<crate::models::LolRecommendationsMatchHistoryParticipantIdentityPlayer>,
}

impl LolRecommendationsMatchHistoryParticipantIdentities {
    pub fn new() -> LolRecommendationsMatchHistoryParticipantIdentities {
        LolRecommendationsMatchHistoryParticipantIdentities {
            participant_id: None,
            player: None,
        }
    }
}


