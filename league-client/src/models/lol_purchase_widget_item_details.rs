/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPurchaseWidgetItemDetails {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "subTitle", skip_serializing_if = "Option::is_none")]
    pub sub_title: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl LolPurchaseWidgetItemDetails {
    pub fn new() -> LolPurchaseWidgetItemDetails {
        LolPurchaseWidgetItemDetails {
            description: None,
            icon_url: None,
            sub_title: None,
            title: None,
        }
    }
}


