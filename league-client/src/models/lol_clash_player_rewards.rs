/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolClashPlayerRewards {
    #[serde(rename = "seasonVp", skip_serializing_if = "Option::is_none")]
    pub season_vp: Option<i32>,
    #[serde(rename = "themeVp", skip_serializing_if = "Option::is_none")]
    pub theme_vp: Option<Vec<crate::models::LolClashThemeVp>>,
}

impl LolClashPlayerRewards {
    pub fn new() -> LolClashPlayerRewards {
        LolClashPlayerRewards {
            season_vp: None,
            theme_vp: None,
        }
    }
}


