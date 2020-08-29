/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolSummonerGameloopPlayerInfoV2 {
    #[serde(rename = "autoFillDataBags", skip_serializing_if = "Option::is_none")]
    pub auto_fill_data_bags: Option<Vec<crate::models::LolSummonerAutoFillQueueDto>>,
    #[serde(rename = "rerollDataBags", skip_serializing_if = "Option::is_none")]
    pub reroll_data_bags: Option<Vec<crate::models::LolSummonerRerollDataBagForClientV1>>,
}

impl LolSummonerGameloopPlayerInfoV2 {
    pub fn new() -> LolSummonerGameloopPlayerInfoV2 {
        LolSummonerGameloopPlayerInfoV2 {
            auto_fill_data_bags: None,
            reroll_data_bags: None,
        }
    }
}


