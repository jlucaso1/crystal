/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolBannersBannerFlag {
    #[serde(rename = "earnedDateIso8601", skip_serializing_if = "Option::is_none")]
    pub earned_date_iso8601: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(rename = "seasonId", skip_serializing_if = "Option::is_none")]
    pub season_id: Option<i64>,
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

impl LolBannersBannerFlag {
    pub fn new() -> LolBannersBannerFlag {
        LolBannersBannerFlag {
            earned_date_iso8601: None,
            item_id: None,
            level: None,
            season_id: None,
            theme: None,
        }
    }
}


