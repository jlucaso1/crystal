/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolSuggestedPlayersSuggestedPlayersReason {
    #[serde(rename = "PreviousPremade")]
    PreviousPremade,
    #[serde(rename = "OnlineFriend")]
    OnlineFriend,
    #[serde(rename = "FriendOfFriend")]
    FriendOfFriend,
    #[serde(rename = "VictoriousComrade")]
    VictoriousComrade,
    #[serde(rename = "LegacyPlayAgain")]
    LegacyPlayAgain,

}




