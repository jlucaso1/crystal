/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcloudVoiceChatDeviceResource {
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(rename = "is_current_device", skip_serializing_if = "Option::is_none")]
    pub is_current_device: Option<bool>,
    #[serde(rename = "is_default", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "is_effective_device", skip_serializing_if = "Option::is_none")]
    pub is_effective_device: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GcloudVoiceChatDeviceResource {
    pub fn new() -> GcloudVoiceChatDeviceResource {
        GcloudVoiceChatDeviceResource {
            handle: None,
            is_current_device: None,
            is_default: None,
            is_effective_device: None,
            name: None,
        }
    }
}


