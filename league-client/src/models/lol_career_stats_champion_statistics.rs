/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCareerStatsChampionStatistics {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "experts", skip_serializing_if = "Option::is_none")]
    pub experts: Option<Vec<crate::models::LolCareerStatsExpertPlayer>>,
    #[serde(rename = "queueStats", skip_serializing_if = "Option::is_none")]
    pub queue_stats: Option<Vec<crate::models::LolCareerStatsStatisticsByQueue>>,
}

impl LolCareerStatsChampionStatistics {
    pub fn new() -> LolCareerStatsChampionStatistics {
        LolCareerStatsChampionStatistics {
            champion_id: None,
            experts: None,
            queue_stats: None,
        }
    }
}


