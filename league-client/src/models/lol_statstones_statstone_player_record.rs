/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolStatstonesStatstonePlayerRecord {
    #[serde(rename = "dateAcquired", skip_serializing_if = "Option::is_none")]
    pub date_acquired: Option<String>,
    #[serde(rename = "dateArchived", skip_serializing_if = "Option::is_none")]
    pub date_archived: Option<String>,
    #[serde(rename = "dateCompleted", skip_serializing_if = "Option::is_none")]
    pub date_completed: Option<String>,
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "entitled", skip_serializing_if = "Option::is_none")]
    pub entitled: Option<bool>,
    #[serde(rename = "milestoneLevel", skip_serializing_if = "Option::is_none")]
    pub milestone_level: Option<i32>,
    #[serde(rename = "personalBest", skip_serializing_if = "Option::is_none")]
    pub personal_best: Option<i32>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "statstoneId", skip_serializing_if = "Option::is_none")]
    pub statstone_id: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl LolStatstonesStatstonePlayerRecord {
    pub fn new() -> LolStatstonesStatstonePlayerRecord {
        LolStatstonesStatstonePlayerRecord {
            date_acquired: None,
            date_archived: None,
            date_completed: None,
            date_modified: None,
            entitled: None,
            milestone_level: None,
            personal_best: None,
            puuid: None,
            statstone_id: None,
            value: None,
        }
    }
}


