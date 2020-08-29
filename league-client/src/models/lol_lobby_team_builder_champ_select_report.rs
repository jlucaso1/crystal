/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyTeamBuilderChampSelectReport {
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "match_token", skip_serializing_if = "Option::is_none")]
    pub match_token: Option<String>,
    #[serde(rename = "offender_puuid", skip_serializing_if = "Option::is_none")]
    pub offender_puuid: Option<String>,
}

impl LolLobbyTeamBuilderChampSelectReport {
    pub fn new() -> LolLobbyTeamBuilderChampSelectReport {
        LolLobbyTeamBuilderChampSelectReport {
            categories: None,
            comment: None,
            location: None,
            match_token: None,
            offender_puuid: None,
        }
    }
}

