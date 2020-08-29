/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolAccountVerificationSendTokenRequest {
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "mediator", skip_serializing_if = "Option::is_none")]
    pub mediator: Option<String>,
}

impl LolAccountVerificationSendTokenRequest {
    pub fn new() -> LolAccountVerificationSendTokenRequest {
        LolAccountVerificationSendTokenRequest {
            device: None,
            locale: None,
            mediator: None,
        }
    }
}


