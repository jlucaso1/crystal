/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceChatStateResource {
    #[serde(rename = "connected", skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
}

impl VoiceChatStateResource {
    pub fn new() -> VoiceChatStateResource {
        VoiceChatStateResource {
            connected: None,
        }
    }
}


