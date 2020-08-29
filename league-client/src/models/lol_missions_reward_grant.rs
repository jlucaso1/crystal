/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolMissionsRewardGrant {
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::models::LolMissionsRewardGrantInfo>,
    #[serde(rename = "rewardGroup", skip_serializing_if = "Option::is_none")]
    pub reward_group: Option<crate::models::LolMissionsRewardGroup>,
}

impl LolMissionsRewardGrant {
    pub fn new() -> LolMissionsRewardGrant {
        LolMissionsRewardGrant {
            info: None,
            reward_group: None,
        }
    }
}

