/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolSuggestedPlayersSuggestedPlayersVictoriousComrade {
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "summonerName", skip_serializing_if = "Option::is_none")]
    pub summoner_name: Option<String>,
}

impl LolSuggestedPlayersSuggestedPlayersVictoriousComrade {
    pub fn new() -> LolSuggestedPlayersSuggestedPlayersVictoriousComrade {
        LolSuggestedPlayersSuggestedPlayersVictoriousComrade {
            summoner_id: None,
            summoner_name: None,
        }
    }
}


