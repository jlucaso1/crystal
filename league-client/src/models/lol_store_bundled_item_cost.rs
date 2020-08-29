/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolStoreBundledItemCost {
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,
    #[serde(rename = "costType", skip_serializing_if = "Option::is_none")]
    pub cost_type: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "discount", skip_serializing_if = "Option::is_none")]
    pub discount: Option<f32>,
}

impl LolStoreBundledItemCost {
    pub fn new() -> LolStoreBundledItemCost {
        LolStoreBundledItemCost {
            cost: None,
            cost_type: None,
            currency: None,
            discount: None,
        }
    }
}


