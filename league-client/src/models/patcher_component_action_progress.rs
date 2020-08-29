/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatcherComponentActionProgress {
    #[serde(rename = "currentItem", skip_serializing_if = "Option::is_none")]
    pub current_item: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::models::PatcherComponentStateProgress>,
    #[serde(rename = "primaryWork", skip_serializing_if = "Option::is_none")]
    pub primary_work: Option<crate::models::PatcherComponentStateWorkType>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::models::PatcherComponentStateProgress>,
}

impl PatcherComponentActionProgress {
    pub fn new() -> PatcherComponentActionProgress {
        PatcherComponentActionProgress {
            current_item: None,
            network: None,
            primary_work: None,
            total: None,
        }
    }
}

