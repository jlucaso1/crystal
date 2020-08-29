/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyEligibility {
    #[serde(rename = "eligible", skip_serializing_if = "Option::is_none")]
    pub eligible: Option<bool>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<crate::models::LolLobbyEligibilityRestriction>>,
}

impl LolLobbyEligibility {
    pub fn new() -> LolLobbyEligibility {
        LolLobbyEligibility {
            eligible: None,
            queue_id: None,
            restrictions: None,
        }
    }
}


