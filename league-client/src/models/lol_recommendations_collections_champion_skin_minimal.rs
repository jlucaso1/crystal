/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolRecommendationsCollectionsChampionSkinMinimal {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isBase", skip_serializing_if = "Option::is_none")]
    pub is_base: Option<bool>,
    #[serde(rename = "lastSelected", skip_serializing_if = "Option::is_none")]
    pub last_selected: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<crate::models::LolRecommendationsCollectionsOwnership>,
    #[serde(rename = "stillObtainable", skip_serializing_if = "Option::is_none")]
    pub still_obtainable: Option<bool>,
}

impl LolRecommendationsCollectionsChampionSkinMinimal {
    pub fn new() -> LolRecommendationsCollectionsChampionSkinMinimal {
        LolRecommendationsCollectionsChampionSkinMinimal {
            champion_id: None,
            disabled: None,
            id: None,
            is_base: None,
            last_selected: None,
            name: None,
            ownership: None,
            still_obtainable: None,
        }
    }
}


