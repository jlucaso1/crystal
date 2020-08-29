/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChampSelectChampSelectTimer {
    #[serde(rename = "adjustedTimeLeftInPhase", skip_serializing_if = "Option::is_none")]
    pub adjusted_time_left_in_phase: Option<i64>,
    #[serde(rename = "internalNowInEpochMs", skip_serializing_if = "Option::is_none")]
    pub internal_now_in_epoch_ms: Option<i64>,
    #[serde(rename = "isInfinite", skip_serializing_if = "Option::is_none")]
    pub is_infinite: Option<bool>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(rename = "totalTimeInPhase", skip_serializing_if = "Option::is_none")]
    pub total_time_in_phase: Option<i64>,
}

impl LolChampSelectChampSelectTimer {
    pub fn new() -> LolChampSelectChampSelectTimer {
        LolChampSelectChampSelectTimer {
            adjusted_time_left_in_phase: None,
            internal_now_in_epoch_ms: None,
            is_infinite: None,
            phase: None,
            total_time_in_phase: None,
        }
    }
}

