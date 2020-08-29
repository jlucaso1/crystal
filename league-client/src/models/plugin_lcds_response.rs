/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginLcdsResponse {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

impl PluginLcdsResponse {
    pub fn new() -> PluginLcdsResponse {
        PluginLcdsResponse {
            body: None,
            type_name: None,
        }
    }
}


