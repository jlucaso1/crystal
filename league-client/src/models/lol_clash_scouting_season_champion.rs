/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolClashScoutingSeasonChampion {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "gameCount", skip_serializing_if = "Option::is_none")]
    pub game_count: Option<i32>,
    #[serde(rename = "kda", skip_serializing_if = "Option::is_none")]
    pub kda: Option<String>,
    #[serde(rename = "kdaClassification", skip_serializing_if = "Option::is_none")]
    pub kda_classification: Option<crate::models::LolClashKdaClassification>,
    #[serde(rename = "winCount", skip_serializing_if = "Option::is_none")]
    pub win_count: Option<i32>,
    #[serde(rename = "winRate", skip_serializing_if = "Option::is_none")]
    pub win_rate: Option<i32>,
}

impl LolClashScoutingSeasonChampion {
    pub fn new() -> LolClashScoutingSeasonChampion {
        LolClashScoutingSeasonChampion {
            champion_id: None,
            game_count: None,
            kda: None,
            kda_classification: None,
            win_count: None,
            win_rate: None,
        }
    }
}


