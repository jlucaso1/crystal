/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolMatchmakingMatchmakingSearchState {
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "AbandonedLowPriorityQueue")]
    AbandonedLowPriorityQueue,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "Searching")]
    Searching,
    #[serde(rename = "Found")]
    Found,
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "ServiceError")]
    ServiceError,
    #[serde(rename = "ServiceShutdown")]
    ServiceShutdown,

}




