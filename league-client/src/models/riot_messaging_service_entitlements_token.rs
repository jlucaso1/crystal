/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RiotMessagingServiceEntitlementsToken {
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "entitlements", skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<String>>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl RiotMessagingServiceEntitlementsToken {
    pub fn new() -> RiotMessagingServiceEntitlementsToken {
        RiotMessagingServiceEntitlementsToken {
            access_token: None,
            entitlements: None,
            issuer: None,
            subject: None,
            token: None,
        }
    }
}


