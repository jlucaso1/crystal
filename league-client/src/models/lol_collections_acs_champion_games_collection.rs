/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCollectionsAcsChampionGamesCollection {
    #[serde(rename = "champions", skip_serializing_if = "Option::is_none")]
    pub champions: Option<Vec<crate::models::LolCollectionsAcsChampionGames>>,
    #[serde(rename = "gameCount", skip_serializing_if = "Option::is_none")]
    pub game_count: Option<i32>,
}

impl LolCollectionsAcsChampionGamesCollection {
    pub fn new() -> LolCollectionsAcsChampionGamesCollection {
        LolCollectionsAcsChampionGamesCollection {
            champions: None,
            game_count: None,
        }
    }
}

