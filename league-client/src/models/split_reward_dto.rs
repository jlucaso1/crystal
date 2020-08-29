/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplitRewardDto {
    #[serde(rename = "defaultRewardId", skip_serializing_if = "Option::is_none")]
    pub default_reward_id: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::SplitRewardsMetaData>,
    #[serde(rename = "rewardType", skip_serializing_if = "Option::is_none")]
    pub reward_type: Option<String>,
    #[serde(rename = "tieredRewardIds", skip_serializing_if = "Option::is_none")]
    pub tiered_reward_ids: Option<::std::collections::HashMap<String, String>>,
}

impl SplitRewardDto {
    pub fn new() -> SplitRewardDto {
        SplitRewardDto {
            default_reward_id: None,
            metadata: None,
            reward_type: None,
            tiered_reward_ids: None,
        }
    }
}

