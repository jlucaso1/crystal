/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChampSelectCollectionsOwnership {
    #[serde(rename = "freeToPlayReward", skip_serializing_if = "Option::is_none")]
    pub free_to_play_reward: Option<bool>,
    #[serde(rename = "owned", skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
    #[serde(rename = "rental", skip_serializing_if = "Option::is_none")]
    pub rental: Option<crate::models::LolChampSelectCollectionsRental>,
}

impl LolChampSelectCollectionsOwnership {
    pub fn new() -> LolChampSelectCollectionsOwnership {
        LolChampSelectCollectionsOwnership {
            free_to_play_reward: None,
            owned: None,
            rental: None,
        }
    }
}

