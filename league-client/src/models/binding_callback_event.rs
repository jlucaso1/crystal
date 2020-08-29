/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BindingCallbackEvent : Represents the parameters of a call to a provided callback.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindingCallbackEvent {
    /// ID of the callback being invoked
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Callback parameters
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<::std::collections::HashMap<String, serde_json::Value>>>,
}

impl BindingCallbackEvent {
    /// Represents the parameters of a call to a provided callback.
    pub fn new() -> BindingCallbackEvent {
        BindingCallbackEvent {
            id: None,
            parameters: None,
        }
    }
}

