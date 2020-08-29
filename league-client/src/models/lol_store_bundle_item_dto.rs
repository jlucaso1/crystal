/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolStoreBundleItemDto {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discountedRp", skip_serializing_if = "Option::is_none")]
    pub discounted_rp: Option<i64>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "inventoryType", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<i64>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owned", skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<i64>,
}

impl LolStoreBundleItemDto {
    pub fn new() -> LolStoreBundleItemDto {
        LolStoreBundleItemDto {
            description: None,
            discounted_rp: None,
            icon_url: None,
            inventory_type: None,
            ip: None,
            item_id: None,
            name: None,
            owned: None,
            quantity: None,
            rp: None,
        }
    }
}

