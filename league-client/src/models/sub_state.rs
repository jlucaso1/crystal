/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubState {
    #[serde(rename = "SUGGESTED")]
    SUGGESTED,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "DECLINED")]
    DECLINED,
    #[serde(rename = "REVOKED")]
    REVOKED,
    #[serde(rename = "NOT_READY")]
    NOTREADY,
    #[serde(rename = "FORCED_NOT_READY")]
    FORCEDNOTREADY,
    #[serde(rename = "READY")]
    READY,

}




