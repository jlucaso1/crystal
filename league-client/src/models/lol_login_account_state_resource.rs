/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLoginAccountStateResource {
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolLoginAccountStateType>,
}

impl LolLoginAccountStateResource {
    pub fn new() -> LolLoginAccountStateResource {
        LolLoginAccountStateResource {
            state: None,
        }
    }
}


