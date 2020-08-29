/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPurchaseWidgetItemCost {
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "discount", skip_serializing_if = "Option::is_none")]
    pub discount: Option<f32>,
}

impl LolPurchaseWidgetItemCost {
    pub fn new() -> LolPurchaseWidgetItemCost {
        LolPurchaseWidgetItemCost {
            cost: None,
            currency: None,
            discount: None,
        }
    }
}


