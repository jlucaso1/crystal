/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyTeamBuilderGatekeeperRestriction {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "remainingMillis", skip_serializing_if = "Option::is_none")]
    pub remaining_millis: Option<i32>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolLobbyTeamBuilderGatekeeperRestriction {
    pub fn new() -> LolLobbyTeamBuilderGatekeeperRestriction {
        LolLobbyTeamBuilderGatekeeperRestriction {
            payload: None,
            queue_id: None,
            reason: None,
            remaining_millis: None,
            summoner_id: None,
        }
    }
}

