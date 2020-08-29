/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChampSelectLegacyChampSelectBannedChampions {
    #[serde(rename = "myTeamBans", skip_serializing_if = "Option::is_none")]
    pub my_team_bans: Option<Vec<i32>>,
    #[serde(rename = "numBans", skip_serializing_if = "Option::is_none")]
    pub num_bans: Option<i32>,
    #[serde(rename = "theirTeamBans", skip_serializing_if = "Option::is_none")]
    pub their_team_bans: Option<Vec<i32>>,
}

impl LolChampSelectLegacyChampSelectBannedChampions {
    pub fn new() -> LolChampSelectLegacyChampSelectBannedChampions {
        LolChampSelectLegacyChampSelectBannedChampions {
            my_team_bans: None,
            num_bans: None,
            their_team_bans: None,
        }
    }
}


