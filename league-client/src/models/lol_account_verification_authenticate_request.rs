/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolAccountVerificationAuthenticateRequest {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl LolAccountVerificationAuthenticateRequest {
    pub fn new() -> LolAccountVerificationAuthenticateRequest {
        LolAccountVerificationAuthenticateRequest {
            password: None,
            username: None,
        }
    }
}


