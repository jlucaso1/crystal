/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolHovercardPatchlineMetadata {
    #[serde(rename = "content_cookies", skip_serializing_if = "Option::is_none")]
    pub content_cookies: Option<Vec<crate::models::LolHovercardContentCookies>>,
    #[serde(rename = "content_paths", skip_serializing_if = "Option::is_none")]
    pub content_paths: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl LolHovercardPatchlineMetadata {
    pub fn new() -> LolHovercardPatchlineMetadata {
        LolHovercardPatchlineMetadata {
            content_cookies: None,
            content_paths: None,
            id: None,
            product_id: None,
        }
    }
}


