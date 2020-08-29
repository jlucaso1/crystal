/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolEsportStreamNotificationsEsportsApiTeams {
    #[serde(rename = "acronym", skip_serializing_if = "Option::is_none")]
    pub acronym: Option<String>,
    #[serde(rename = "guid", skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "logoUrl", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl LolEsportStreamNotificationsEsportsApiTeams {
    pub fn new() -> LolEsportStreamNotificationsEsportsApiTeams {
        LolEsportStreamNotificationsEsportsApiTeams {
            acronym: None,
            guid: None,
            id: None,
            logo_url: None,
            name: None,
            slug: None,
        }
    }
}

