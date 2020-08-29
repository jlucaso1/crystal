/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesEventEndV1 {
    /// event name
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// optional event name suffix
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// timestamp in microseconds of when the event occurred
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<i64>,
}

impl TimeSeriesEventEndV1 {
    pub fn new() -> TimeSeriesEventEndV1 {
        TimeSeriesEventEndV1 {
            event_name: None,
            suffix: None,
            when: None,
        }
    }
}


