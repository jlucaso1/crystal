/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolNpeTutorialPathCollectionsChampion {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "banVoPath", skip_serializing_if = "Option::is_none")]
    pub ban_vo_path: Option<String>,
    #[serde(rename = "chooseVoPath", skip_serializing_if = "Option::is_none")]
    pub choose_vo_path: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "passive", skip_serializing_if = "Option::is_none")]
    pub passive: Option<crate::models::LolNpeTutorialPathCollectionsChampionSpell>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(rename = "spells", skip_serializing_if = "Option::is_none")]
    pub spells: Option<Vec<crate::models::LolNpeTutorialPathCollectionsChampionSpell>>,
    #[serde(rename = "squarePortraitPath", skip_serializing_if = "Option::is_none")]
    pub square_portrait_path: Option<String>,
    #[serde(rename = "stingerSfxPath", skip_serializing_if = "Option::is_none")]
    pub stinger_sfx_path: Option<String>,
}

impl LolNpeTutorialPathCollectionsChampion {
    pub fn new() -> LolNpeTutorialPathCollectionsChampion {
        LolNpeTutorialPathCollectionsChampion {
            alias: None,
            ban_vo_path: None,
            choose_vo_path: None,
            id: None,
            name: None,
            passive: None,
            roles: None,
            spells: None,
            square_portrait_path: None,
            stinger_sfx_path: None,
        }
    }
}


