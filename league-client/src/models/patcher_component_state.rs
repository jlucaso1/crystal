/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatcherComponentState {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::models::PatcherComponentStateAction>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isCorrupted", skip_serializing_if = "Option::is_none")]
    pub is_corrupted: Option<bool>,
    #[serde(rename = "isUpToDate", skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<bool>,
    #[serde(rename = "isUpdateAvailable", skip_serializing_if = "Option::is_none")]
    pub is_update_available: Option<bool>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<crate::models::PatcherComponentActionProgress>,
    #[serde(rename = "timeOfLastUpToDateCheckISO8601", skip_serializing_if = "Option::is_none")]
    pub time_of_last_up_to_date_check_iso8601: Option<String>,
}

impl PatcherComponentState {
    pub fn new() -> PatcherComponentState {
        PatcherComponentState {
            action: None,
            id: None,
            is_corrupted: None,
            is_up_to_date: None,
            is_update_available: None,
            progress: None,
            time_of_last_up_to_date_check_iso8601: None,
        }
    }
}


