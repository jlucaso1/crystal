/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolGameflowGameflowPhase {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Lobby")]
    Lobby,
    #[serde(rename = "Matchmaking")]
    Matchmaking,
    #[serde(rename = "CheckedIntoTournament")]
    CheckedIntoTournament,
    #[serde(rename = "ReadyCheck")]
    ReadyCheck,
    #[serde(rename = "ChampSelect")]
    ChampSelect,
    #[serde(rename = "GameStart")]
    GameStart,
    #[serde(rename = "FailedToLaunch")]
    FailedToLaunch,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Reconnect")]
    Reconnect,
    #[serde(rename = "WaitingForStats")]
    WaitingForStats,
    #[serde(rename = "PreEndOfGame")]
    PreEndOfGame,
    #[serde(rename = "EndOfGame")]
    EndOfGame,
    #[serde(rename = "TerminatedInError")]
    TerminatedInError,

}




