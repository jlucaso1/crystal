/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolRsoAuthAuthorizationRequest {
    #[serde(rename = "claims", skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<String>>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(rename = "trustLevels", skip_serializing_if = "Option::is_none")]
    pub trust_levels: Option<Vec<crate::models::LolRsoAuthRsoAuthorizationTrustLevel>>,
}

impl LolRsoAuthAuthorizationRequest {
    pub fn new() -> LolRsoAuthAuthorizationRequest {
        LolRsoAuthAuthorizationRequest {
            claims: None,
            client_id: None,
            scope: None,
            trust_levels: None,
        }
    }
}

