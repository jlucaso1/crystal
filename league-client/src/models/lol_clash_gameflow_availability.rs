/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolClashGameflowAvailability {
    #[serde(rename = "isAvailable", skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
}

impl LolClashGameflowAvailability {
    pub fn new() -> LolClashGameflowAvailability {
        LolClashGameflowAvailability {
            is_available: None,
        }
    }
}


