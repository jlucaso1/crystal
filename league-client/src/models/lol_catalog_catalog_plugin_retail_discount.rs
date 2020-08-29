/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCatalogCatalogPluginRetailDiscount {
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,
    #[serde(rename = "discount", skip_serializing_if = "Option::is_none")]
    pub discount: Option<f32>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl LolCatalogCatalogPluginRetailDiscount {
    pub fn new() -> LolCatalogCatalogPluginRetailDiscount {
        LolCatalogCatalogPluginRetailDiscount {
            cost: None,
            discount: None,
            end_date: None,
            start_date: None,
        }
    }
}

