/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPlayerBehaviorPlayerBehaviorGameflowSessionResource {
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<crate::models::LolPlayerBehaviorGameflowPhase>,
}

impl LolPlayerBehaviorPlayerBehaviorGameflowSessionResource {
    pub fn new() -> LolPlayerBehaviorPlayerBehaviorGameflowSessionResource {
        LolPlayerBehaviorPlayerBehaviorGameflowSessionResource {
            phase: None,
        }
    }
}


