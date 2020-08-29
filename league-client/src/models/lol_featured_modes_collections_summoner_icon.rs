/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolFeaturedModesCollectionsSummonerIcon {
    #[serde(rename = "iconId", skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<i32>,
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<crate::models::LolFeaturedModesCollectionsOwnership>,
}

impl LolFeaturedModesCollectionsSummonerIcon {
    pub fn new() -> LolFeaturedModesCollectionsSummonerIcon {
        LolFeaturedModesCollectionsSummonerIcon {
            icon_id: None,
            ownership: None,
        }
    }
}


