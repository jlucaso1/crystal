/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceChatVoiceErrorResource {
    #[serde(rename = "errorString", skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
    #[serde(rename = "responseType", skip_serializing_if = "Option::is_none")]
    pub response_type: Option<i64>,
    #[serde(rename = "responseTypeString", skip_serializing_if = "Option::is_none")]
    pub response_type_string: Option<String>,
    #[serde(rename = "returnCode", skip_serializing_if = "Option::is_none")]
    pub return_code: Option<i64>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    #[serde(rename = "statusString", skip_serializing_if = "Option::is_none")]
    pub status_string: Option<String>,
}

impl VoiceChatVoiceErrorResource {
    pub fn new() -> VoiceChatVoiceErrorResource {
        VoiceChatVoiceErrorResource {
            error_string: None,
            response_type: None,
            response_type_string: None,
            return_code: None,
            status_code: None,
            status_string: None,
        }
    }
}

