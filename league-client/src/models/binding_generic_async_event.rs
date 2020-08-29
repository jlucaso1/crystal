/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BindingGenericAsyncEvent : Represents generic data for an asynchronous event.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindingGenericAsyncEvent {
    /// Asynchronous operation token
    #[serde(rename = "asyncToken", skip_serializing_if = "Option::is_none")]
    pub async_token: Option<i32>,
    /// Event data
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl BindingGenericAsyncEvent {
    /// Represents generic data for an asynchronous event.
    pub fn new() -> BindingGenericAsyncEvent {
        BindingGenericAsyncEvent {
            async_token: None,
            data: None,
        }
    }
}


