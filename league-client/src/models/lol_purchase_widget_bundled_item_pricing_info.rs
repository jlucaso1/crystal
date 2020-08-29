/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPurchaseWidgetBundledItemPricingInfo {
    #[serde(rename = "discountPrices", skip_serializing_if = "Option::is_none")]
    pub discount_prices: Option<Vec<crate::models::LolPurchaseWidgetDiscountPricingInfo>>,
    #[serde(rename = "inventoryType", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl LolPurchaseWidgetBundledItemPricingInfo {
    pub fn new() -> LolPurchaseWidgetBundledItemPricingInfo {
        LolPurchaseWidgetBundledItemPricingInfo {
            discount_prices: None,
            inventory_type: None,
            item_id: None,
            quantity: None,
        }
    }
}


