/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPurchaseWidgetTransaction {
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "itemKey", skip_serializing_if = "Option::is_none")]
    pub item_key: Option<crate::models::LolPurchaseWidgetItemKey>,
    #[serde(rename = "itemName", skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl LolPurchaseWidgetTransaction {
    pub fn new() -> LolPurchaseWidgetTransaction {
        LolPurchaseWidgetTransaction {
            icon_url: None,
            item_key: None,
            item_name: None,
            transaction_id: None,
        }
    }
}


