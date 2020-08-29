/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LcdsBotParticipant {
    #[serde(rename = "botSkillLevel", skip_serializing_if = "Option::is_none")]
    pub bot_skill_level: Option<i32>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "summonerInternalName", skip_serializing_if = "Option::is_none")]
    pub summoner_internal_name: Option<String>,
    #[serde(rename = "summonerName", skip_serializing_if = "Option::is_none")]
    pub summoner_name: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

impl LcdsBotParticipant {
    pub fn new() -> LcdsBotParticipant {
        LcdsBotParticipant {
            bot_skill_level: None,
            summoner_id: None,
            summoner_internal_name: None,
            summoner_name: None,
            team_id: None,
        }
    }
}


