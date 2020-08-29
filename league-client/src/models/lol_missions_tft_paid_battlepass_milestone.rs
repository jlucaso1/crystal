/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolMissionsTftPaidBattlepassMilestone {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "internalName", skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "isBonus", skip_serializing_if = "Option::is_none")]
    pub is_bonus: Option<bool>,
    #[serde(rename = "isKeystone", skip_serializing_if = "Option::is_none")]
    pub is_keystone: Option<bool>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isPaid", skip_serializing_if = "Option::is_none")]
    pub is_paid: Option<bool>,
    #[serde(rename = "largeIconImageUrl", skip_serializing_if = "Option::is_none")]
    pub large_icon_image_url: Option<String>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(rename = "missionId", skip_serializing_if = "Option::is_none")]
    pub mission_id: Option<String>,
    #[serde(rename = "pointsEarnedForMilestone", skip_serializing_if = "Option::is_none")]
    pub points_earned_for_milestone: Option<i32>,
    #[serde(rename = "pointsNeededForMilestone", skip_serializing_if = "Option::is_none")]
    pub points_needed_for_milestone: Option<i32>,
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<crate::models::PlayerMissionRewardDto>>,
    #[serde(rename = "smallIconImageUrl", skip_serializing_if = "Option::is_none")]
    pub small_icon_image_url: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "totalPointsForMilestone", skip_serializing_if = "Option::is_none")]
    pub total_points_for_milestone: Option<i32>,
}

impl LolMissionsTftPaidBattlepassMilestone {
    pub fn new() -> LolMissionsTftPaidBattlepassMilestone {
        LolMissionsTftPaidBattlepassMilestone {
            description: None,
            internal_name: None,
            is_bonus: None,
            is_keystone: None,
            is_locked: None,
            is_paid: None,
            large_icon_image_url: None,
            level: None,
            mission_id: None,
            points_earned_for_milestone: None,
            points_needed_for_milestone: None,
            rewards: None,
            small_icon_image_url: None,
            state: None,
            status: None,
            title: None,
            total_points_for_milestone: None,
        }
    }
}


