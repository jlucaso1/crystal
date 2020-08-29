/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracingPhaseEndV1 {
    /// phase name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// timestamp in microseconds of when the phase ended
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<i64>,
}

impl TracingPhaseEndV1 {
    pub fn new() -> TracingPhaseEndV1 {
        TracingPhaseEndV1 {
            name: None,
            when: None,
        }
    }
}


