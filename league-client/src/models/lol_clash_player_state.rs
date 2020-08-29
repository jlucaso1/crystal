/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolClashPlayerState {
    #[serde(rename = "NO_ROSTER")]
    NOROSTER,
    #[serde(rename = "PENDING_ROSTER")]
    PENDINGROSTER,
    #[serde(rename = "REGISTERED_ROSTER")]
    REGISTEREDROSTER,
    #[serde(rename = "BRACKET_ROSTER")]
    BRACKETROSTER,
    #[serde(rename = "ELIMINATED")]
    ELIMINATED,

}



