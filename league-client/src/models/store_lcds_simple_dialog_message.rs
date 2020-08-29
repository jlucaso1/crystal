/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoreLcdsSimpleDialogMessage {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "msgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl StoreLcdsSimpleDialogMessage {
    pub fn new() -> StoreLcdsSimpleDialogMessage {
        StoreLcdsSimpleDialogMessage {
            account_id: None,
            msg_id: None,
            params: None,
            _type: None,
        }
    }
}


