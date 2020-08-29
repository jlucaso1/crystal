/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCosmeticsCompanionsGroupViewModel {
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::LolCosmeticsCosmeticsCompanionViewModel>>,
    #[serde(rename = "numAvailable", skip_serializing_if = "Option::is_none")]
    pub num_available: Option<i32>,
    #[serde(rename = "numOwned", skip_serializing_if = "Option::is_none")]
    pub num_owned: Option<i32>,
}

impl LolCosmeticsCompanionsGroupViewModel {
    pub fn new() -> LolCosmeticsCompanionsGroupViewModel {
        LolCosmeticsCompanionsGroupViewModel {
            group_id: None,
            group_name: None,
            items: None,
            num_available: None,
            num_owned: None,
        }
    }
}


