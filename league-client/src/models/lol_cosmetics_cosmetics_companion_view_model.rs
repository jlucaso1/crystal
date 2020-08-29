/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCosmeticsCosmeticsCompanionViewModel {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(rename = "loadoutsIcon", skip_serializing_if = "Option::is_none")]
    pub loadouts_icon: Option<String>,
    #[serde(rename = "loyalty", skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "offerData", skip_serializing_if = "Option::is_none")]
    pub offer_data: Option<crate::models::LolCosmeticsCapOffer>,
    #[serde(rename = "owned", skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
    #[serde(rename = "purchaseDate", skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<String>,
    #[serde(rename = "rarityValue", skip_serializing_if = "Option::is_none")]
    pub rarity_value: Option<i32>,
    #[serde(rename = "selected", skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[serde(rename = "species", skip_serializing_if = "Option::is_none")]
    pub species: Option<String>,
    #[serde(rename = "upgrades", skip_serializing_if = "Option::is_none")]
    pub upgrades: Option<Vec<crate::models::LolCosmeticsCosmeticsCompanionViewModel>>,
}

impl LolCosmeticsCosmeticsCompanionViewModel {
    pub fn new() -> LolCosmeticsCosmeticsCompanionViewModel {
        LolCosmeticsCosmeticsCompanionViewModel {
            color: None,
            content_id: None,
            description: None,
            group_id: None,
            item_id: None,
            level: None,
            loadouts_icon: None,
            loyalty: None,
            name: None,
            offer_data: None,
            owned: None,
            purchase_date: None,
            rarity_value: None,
            selected: None,
            species: None,
            upgrades: None,
        }
    }
}


