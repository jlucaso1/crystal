/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyTeamBuilderChampionBenchStateV1 {
    #[serde(rename = "benchEnabled", skip_serializing_if = "Option::is_none")]
    pub bench_enabled: Option<bool>,
    #[serde(rename = "championIds", skip_serializing_if = "Option::is_none")]
    pub champion_ids: Option<Vec<i32>>,
}

impl LolLobbyTeamBuilderChampionBenchStateV1 {
    pub fn new() -> LolLobbyTeamBuilderChampionBenchStateV1 {
        LolLobbyTeamBuilderChampionBenchStateV1 {
            bench_enabled: None,
            champion_ids: None,
        }
    }
}


