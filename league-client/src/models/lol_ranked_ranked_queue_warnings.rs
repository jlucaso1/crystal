/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolRankedRankedQueueWarnings {
    #[serde(rename = "apexDaysUntilDecay", skip_serializing_if = "Option::is_none")]
    pub apex_days_until_decay: Option<i32>,
    #[serde(rename = "demotionWarning", skip_serializing_if = "Option::is_none")]
    pub demotion_warning: Option<i32>,
    #[serde(rename = "displayDecayWarning", skip_serializing_if = "Option::is_none")]
    pub display_decay_warning: Option<bool>,
    #[serde(rename = "timeUntilInactivityStatusChanges", skip_serializing_if = "Option::is_none")]
    pub time_until_inactivity_status_changes: Option<i64>,
}

impl LolRankedRankedQueueWarnings {
    pub fn new() -> LolRankedRankedQueueWarnings {
        LolRankedRankedQueueWarnings {
            apex_days_until_decay: None,
            demotion_warning: None,
            display_decay_warning: None,
            time_until_inactivity_status_changes: None,
        }
    }
}


