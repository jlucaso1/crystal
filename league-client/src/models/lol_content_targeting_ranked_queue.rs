/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolContentTargetingRankedQueue {
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "RANKED_SOLO_5x5")]
    RANKEDSOLO5x5,
    #[serde(rename = "RANKED_FLEX_SR")]
    RANKEDFLEXSR,
    #[serde(rename = "RANKED_FLEX_TT")]
    RANKEDFLEXTT,
    #[serde(rename = "RANKED_TFT")]
    RANKEDTFT,

}




