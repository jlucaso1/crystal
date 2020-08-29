/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MissionProgressDto {
    #[serde(rename = "currentProgress", skip_serializing_if = "Option::is_none")]
    pub current_progress: Option<i32>,
    #[serde(rename = "lastViewedProgress", skip_serializing_if = "Option::is_none")]
    pub last_viewed_progress: Option<i32>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl MissionProgressDto {
    pub fn new() -> MissionProgressDto {
        MissionProgressDto {
            current_progress: None,
            last_viewed_progress: None,
            total_count: None,
        }
    }
}


