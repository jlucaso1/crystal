/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolSuggestedPlayersSuggestedPlayersLobbyStatus {
    #[serde(rename = "invitedSummonerIds", skip_serializing_if = "Option::is_none")]
    pub invited_summoner_ids: Option<Vec<i64>>,
    #[serde(rename = "memberSummonerIds", skip_serializing_if = "Option::is_none")]
    pub member_summoner_ids: Option<Vec<i64>>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
}

impl LolSuggestedPlayersSuggestedPlayersLobbyStatus {
    pub fn new() -> LolSuggestedPlayersSuggestedPlayersLobbyStatus {
        LolSuggestedPlayersSuggestedPlayersLobbyStatus {
            invited_summoner_ids: None,
            member_summoner_ids: None,
            queue_id: None,
        }
    }
}


