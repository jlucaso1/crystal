/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BindingAsyncState : Possible states of an asynchronous operation.

/// Possible states of an asynchronous operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BindingAsyncState {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Cancelling")]
    Cancelling,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Succeeded")]
    Succeeded,
    #[serde(rename = "Failed")]
    Failed,

}




