/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricMetadataNotify {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    #[serde(rename = "hipchat", skip_serializing_if = "Option::is_none")]
    pub hipchat: Option<Vec<crate::models::MetricMetadataHipchatNotification>>,
    #[serde(rename = "pagerduty", skip_serializing_if = "Option::is_none")]
    pub pagerduty: Option<Vec<crate::models::MetricMetadataPagerDutyNotification>>,
}

impl MetricMetadataNotify {
    pub fn new() -> MetricMetadataNotify {
        MetricMetadataNotify {
            email: None,
            hipchat: None,
            pagerduty: None,
        }
    }
}


