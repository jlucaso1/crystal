/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLeaverBusterMatchmakingSearchResource {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::LolLeaverBusterMatchmakingSearchErrorResource>>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
}

impl LolLeaverBusterMatchmakingSearchResource {
    pub fn new() -> LolLeaverBusterMatchmakingSearchResource {
        LolLeaverBusterMatchmakingSearchResource {
            errors: None,
            queue_id: None,
        }
    }
}

