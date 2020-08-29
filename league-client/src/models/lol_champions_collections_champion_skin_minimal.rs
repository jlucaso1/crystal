/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChampionsCollectionsChampionSkinMinimal {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "chromaPath", skip_serializing_if = "Option::is_none")]
    pub chroma_path: Option<String>,
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
    pub ownership: Option<crate::models::LolChampionsCollectionsOwnership>,
    #[serde(rename = "splashPath", skip_serializing_if = "Option::is_none")]
    pub splash_path: Option<String>,
    #[serde(rename = "stillObtainable", skip_serializing_if = "Option::is_none")]
    pub still_obtainable: Option<bool>,
    #[serde(rename = "tilePath", skip_serializing_if = "Option::is_none")]
    pub tile_path: Option<String>,
}

impl LolChampionsCollectionsChampionSkinMinimal {
    pub fn new() -> LolChampionsCollectionsChampionSkinMinimal {
        LolChampionsCollectionsChampionSkinMinimal {
            champion_id: None,
            chroma_path: None,
            disabled: None,
            id: None,
            is_base: None,
            last_selected: None,
            name: None,
            ownership: None,
            splash_path: None,
            still_obtainable: None,
            tile_path: None,
        }
    }
}


