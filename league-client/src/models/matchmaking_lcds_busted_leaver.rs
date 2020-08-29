/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchmakingLcdsBustedLeaver {
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "leaverPenaltyMillisRemaining", skip_serializing_if = "Option::is_none")]
    pub leaver_penalty_millis_remaining: Option<i64>,
    #[serde(rename = "reasonFailed", skip_serializing_if = "Option::is_none")]
    pub reason_failed: Option<String>,
    #[serde(rename = "summoner", skip_serializing_if = "Option::is_none")]
    pub summoner: Option<crate::models::MatchmakingLcdsSummoner>,
}

impl MatchmakingLcdsBustedLeaver {
    pub fn new() -> MatchmakingLcdsBustedLeaver {
        MatchmakingLcdsBustedLeaver {
            access_token: None,
            leaver_penalty_millis_remaining: None,
            reason_failed: None,
            summoner: None,
        }
    }
}


