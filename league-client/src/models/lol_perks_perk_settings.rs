/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPerksPerkSettings {
    #[serde(rename = "pages", skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<crate::models::LolPerksPerkPageResource>>,
    #[serde(rename = "perShardPerkBooks", skip_serializing_if = "Option::is_none")]
    pub per_shard_perk_books: Option<::std::collections::HashMap<String, crate::models::LolPerksPerkBook>>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<crate::models::LolPerksUiSettings>,
}

impl LolPerksPerkSettings {
    pub fn new() -> LolPerksPerkSettings {
        LolPerksPerkSettings {
            pages: None,
            per_shard_perk_books: None,
            settings: None,
        }
    }
}

