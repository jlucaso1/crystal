/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolKickoutKickoutMessage {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl LolKickoutKickoutMessage {
    pub fn new() -> LolKickoutKickoutMessage {
        LolKickoutKickoutMessage {
            message: None,
        }
    }
}

