/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolPatchComponentStateAction {
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "CheckingForUpdates")]
    CheckingForUpdates,
    #[serde(rename = "Patching")]
    Patching,
    #[serde(rename = "Repairing")]
    Repairing,
    #[serde(rename = "Migrating")]
    Migrating,

}




