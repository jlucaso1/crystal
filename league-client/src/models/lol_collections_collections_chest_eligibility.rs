/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCollectionsCollectionsChestEligibility {
    #[serde(rename = "earnableChests", skip_serializing_if = "Option::is_none")]
    pub earnable_chests: Option<i32>,
    #[serde(rename = "maximumChests", skip_serializing_if = "Option::is_none")]
    pub maximum_chests: Option<i32>,
    #[serde(rename = "nextChestRechargeTime", skip_serializing_if = "Option::is_none")]
    pub next_chest_recharge_time: Option<i64>,
}

impl LolCollectionsCollectionsChestEligibility {
    pub fn new() -> LolCollectionsCollectionsChestEligibility {
        LolCollectionsCollectionsChestEligibility {
            earnable_chests: None,
            maximum_chests: None,
            next_chest_recharge_time: None,
        }
    }
}


