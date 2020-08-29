/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolMissionsCollectionsChampion {
    #[serde(rename = "freeToPlay", skip_serializing_if = "Option::is_none")]
    pub free_to_play: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<crate::models::LolMissionsCollectionsOwnership>,
    #[serde(rename = "skins", skip_serializing_if = "Option::is_none")]
    pub skins: Option<Vec<crate::models::LolMissionsCollectionsChampionSkin>>,
}

impl LolMissionsCollectionsChampion {
    pub fn new() -> LolMissionsCollectionsChampion {
        LolMissionsCollectionsChampion {
            free_to_play: None,
            id: None,
            ownership: None,
            skins: None,
        }
    }
}


