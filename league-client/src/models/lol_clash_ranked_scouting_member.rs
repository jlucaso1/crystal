/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolClashRankedScoutingMember {
    #[serde(rename = "championScoutingData", skip_serializing_if = "Option::is_none")]
    pub champion_scouting_data: Option<Vec<crate::models::LolClashRankedScoutingTopChampion>>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
}

impl LolClashRankedScoutingMember {
    pub fn new() -> LolClashRankedScoutingMember {
        LolClashRankedScoutingMember {
            champion_scouting_data: None,
            player_id: None,
        }
    }
}


