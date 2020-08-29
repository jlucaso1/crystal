/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPerksPlatformConfig {
    #[serde(rename = "AutoRepairPagesEnabled", skip_serializing_if = "Option::is_none")]
    pub auto_repair_pages_enabled: Option<bool>,
    #[serde(rename = "PerksEnabled", skip_serializing_if = "Option::is_none")]
    pub perks_enabled: Option<bool>,
}

impl LolPerksPlatformConfig {
    pub fn new() -> LolPerksPlatformConfig {
        LolPerksPlatformConfig {
            auto_repair_pages_enabled: None,
            perks_enabled: None,
        }
    }
}


