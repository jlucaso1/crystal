/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolHovercardProductMetadata {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "patchlines", skip_serializing_if = "Option::is_none")]
    pub patchlines: Option<::std::collections::HashMap<String, crate::models::LolHovercardPatchlineMetadata>>,
}

impl LolHovercardProductMetadata {
    pub fn new() -> LolHovercardProductMetadata {
        LolHovercardProductMetadata {
            id: None,
            name: None,
            patchlines: None,
        }
    }
}


