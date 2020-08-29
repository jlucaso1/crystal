/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndOfGameLcdsTeamInfo {
    #[serde(rename = "memberStatusString", skip_serializing_if = "Option::is_none")]
    pub member_status_string: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "secondsUntilEligibleForDeletion", skip_serializing_if = "Option::is_none")]
    pub seconds_until_eligible_for_deletion: Option<i64>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<crate::models::EndOfGameLcdsTeamId>,
}

impl EndOfGameLcdsTeamInfo {
    pub fn new() -> EndOfGameLcdsTeamInfo {
        EndOfGameLcdsTeamInfo {
            member_status_string: None,
            name: None,
            seconds_until_eligible_for_deletion: None,
            tag: None,
            team_id: None,
        }
    }
}


