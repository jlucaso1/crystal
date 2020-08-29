/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPatchStatus {
    #[serde(rename = "connectedToPatchServer", skip_serializing_if = "Option::is_none")]
    pub connected_to_patch_server: Option<bool>,
    #[serde(rename = "hasUpdatesOnRestart", skip_serializing_if = "Option::is_none")]
    pub has_updates_on_restart: Option<bool>,
    #[serde(rename = "willRestartOnSelfUpdate", skip_serializing_if = "Option::is_none")]
    pub will_restart_on_self_update: Option<bool>,
}

impl LolPatchStatus {
    pub fn new() -> LolPatchStatus {
        LolPatchStatus {
            connected_to_patch_server: None,
            has_updates_on_restart: None,
            will_restart_on_self_update: None,
        }
    }
}


