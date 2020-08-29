/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCosmeticsTftMapSkinGroupedViewModel {
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<i32>,
    #[serde(rename = "defaultItemId", skip_serializing_if = "Option::is_none")]
    pub default_item_id: Option<i32>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::LolCosmeticsTftMapSkinGroupViewModel>>,
    #[serde(rename = "selectedLoadoutItem", skip_serializing_if = "Option::is_none")]
    pub selected_loadout_item: Option<crate::models::LolCosmeticsCosmeticsTftMapSkinViewModel>,
}

impl LolCosmeticsTftMapSkinGroupedViewModel {
    pub fn new() -> LolCosmeticsTftMapSkinGroupedViewModel {
        LolCosmeticsTftMapSkinGroupedViewModel {
            balance: None,
            default_item_id: None,
            groups: None,
            selected_loadout_item: None,
        }
    }
}


