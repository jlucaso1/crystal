/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultipleReplayMetadataResponseItemV2 {
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::ReplayMetadataV2>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ReplayResponseStatus>,
}

impl MultipleReplayMetadataResponseItemV2 {
    pub fn new() -> MultipleReplayMetadataResponseItemV2 {
        MultipleReplayMetadataResponseItemV2 {
            game_id: None,
            metadata: None,
            status: None,
        }
    }
}

