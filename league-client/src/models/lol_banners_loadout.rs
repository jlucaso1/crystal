/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolBannersLoadout {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "loadout", skip_serializing_if = "Option::is_none")]
    pub loadout: Option<::std::collections::HashMap<String, crate::models::LolBannersLoadoutsSlot>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl LolBannersLoadout {
    pub fn new() -> LolBannersLoadout {
        LolBannersLoadout {
            id: None,
            loadout: None,
            name: None,
            scope: None,
        }
    }
}

