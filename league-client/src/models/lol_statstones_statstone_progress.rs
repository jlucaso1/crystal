/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolStatstonesStatstoneProgress {
    #[serde(rename = "delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    #[serde(rename = "existingProgressPercent", skip_serializing_if = "Option::is_none")]
    pub existing_progress_percent: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "newMilestoneDifference", skip_serializing_if = "Option::is_none")]
    pub new_milestone_difference: Option<String>,
    #[serde(rename = "newProgressPercent", skip_serializing_if = "Option::is_none")]
    pub new_progress_percent: Option<String>,
    #[serde(rename = "nextMilestone", skip_serializing_if = "Option::is_none")]
    pub next_milestone: Option<String>,
    #[serde(rename = "statstoneId", skip_serializing_if = "Option::is_none")]
    pub statstone_id: Option<String>,
    #[serde(rename = "statstoneName", skip_serializing_if = "Option::is_none")]
    pub statstone_name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LolStatstonesStatstoneProgress {
    pub fn new() -> LolStatstonesStatstoneProgress {
        LolStatstonesStatstoneProgress {
            delta: None,
            existing_progress_percent: None,
            image_url: None,
            new_milestone_difference: None,
            new_progress_percent: None,
            next_milestone: None,
            statstone_id: None,
            statstone_name: None,
            value: None,
        }
    }
}


