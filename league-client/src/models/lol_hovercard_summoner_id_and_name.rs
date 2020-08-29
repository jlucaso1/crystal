/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolHovercardSummonerIdAndName {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolHovercardSummonerIdAndName {
    pub fn new() -> LolHovercardSummonerIdAndName {
        LolHovercardSummonerIdAndName {
            display_name: None,
            summoner_id: None,
        }
    }
}


