/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolEsportStreamNotificationsLiveMatch {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "streamGroup", skip_serializing_if = "Option::is_none")]
    pub stream_group: Option<String>,
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<crate::models::LolEsportStreamNotificationsLiveMatchTeam>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tournamentDescription", skip_serializing_if = "Option::is_none")]
    pub tournament_description: Option<String>,
}

impl LolEsportStreamNotificationsLiveMatch {
    pub fn new() -> LolEsportStreamNotificationsLiveMatch {
        LolEsportStreamNotificationsLiveMatch {
            id: None,
            stream_group: None,
            teams: None,
            title: None,
            tournament_description: None,
        }
    }
}

