/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolReplaysReplayContextData {
    #[serde(rename = "componentType", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
}

impl LolReplaysReplayContextData {
    pub fn new() -> LolReplaysReplayContextData {
        LolReplaysReplayContextData {
            component_type: None,
        }
    }
}

