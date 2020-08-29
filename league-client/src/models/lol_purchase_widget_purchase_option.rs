/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPurchaseWidgetPurchaseOption {
    #[serde(rename = "priceDetails", skip_serializing_if = "Option::is_none")]
    pub price_details: Option<Vec<crate::models::LolPurchaseWidgetPriceDetail>>,
}

impl LolPurchaseWidgetPurchaseOption {
    pub fn new() -> LolPurchaseWidgetPurchaseOption {
        LolPurchaseWidgetPurchaseOption {
            price_details: None,
        }
    }
}

