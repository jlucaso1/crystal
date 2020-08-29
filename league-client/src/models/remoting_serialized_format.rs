/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RemotingSerializedFormat : Serialization format for remoting requests and results.

/// Serialization format for remoting requests and results.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RemotingSerializedFormat {
    #[serde(rename = "JSON")]
    JSON,
    #[serde(rename = "YAML")]
    YAML,
    #[serde(rename = "MsgPack")]
    MsgPack,

}




