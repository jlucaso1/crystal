/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolLobbyTeamBuilderLobbyRemovedFromGameReason {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Kicked")]
    Kicked,
    #[serde(rename = "Disbanded")]
    Disbanded,
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "ServiceError")]
    ServiceError,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Timeout")]
    Timeout,
    #[serde(rename = "GameStartError")]
    GameStartError,
    #[serde(rename = "ServiceShutdown")]
    ServiceShutdown,

}




