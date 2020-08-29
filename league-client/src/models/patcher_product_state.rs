/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatcherProductState {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::models::PatcherComponentStateAction>,
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<crate::models::PatcherComponentState>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isCorrupted", skip_serializing_if = "Option::is_none")]
    pub is_corrupted: Option<bool>,
    #[serde(rename = "isStopped", skip_serializing_if = "Option::is_none")]
    pub is_stopped: Option<bool>,
    #[serde(rename = "isUpToDate", skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<bool>,
    #[serde(rename = "isUpdateAvailable", skip_serializing_if = "Option::is_none")]
    pub is_update_available: Option<bool>,
    #[serde(rename = "percentPatched", skip_serializing_if = "Option::is_none")]
    pub percent_patched: Option<f64>,
}

impl PatcherProductState {
    pub fn new() -> PatcherProductState {
        PatcherProductState {
            action: None,
            components: None,
            id: None,
            is_corrupted: None,
            is_stopped: None,
            is_up_to_date: None,
            is_update_available: None,
            percent_patched: None,
        }
    }
}


