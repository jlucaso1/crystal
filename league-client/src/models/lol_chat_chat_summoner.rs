/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatChatSummoner {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "profileIconId", skip_serializing_if = "Option::is_none")]
    pub profile_icon_id: Option<i32>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "summonerLevel", skip_serializing_if = "Option::is_none")]
    pub summoner_level: Option<i32>,
}

impl LolChatChatSummoner {
    pub fn new() -> LolChatChatSummoner {
        LolChatChatSummoner {
            display_name: None,
            profile_icon_id: None,
            puuid: None,
            summoner_id: None,
            summoner_level: None,
        }
    }
}


