/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolClashSimpleStateFlag {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::LolClashSimpleStateStatus>,
}

impl LolClashSimpleStateFlag {
    pub fn new() -> LolClashSimpleStateFlag {
        LolClashSimpleStateFlag {
            id: None,
            status: None,
        }
    }
}


