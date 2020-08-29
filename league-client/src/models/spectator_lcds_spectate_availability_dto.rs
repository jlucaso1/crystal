/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpectatorLcdsSpectateAvailabilityDto {
    #[serde(rename = "teamOrSummonerIds", skip_serializing_if = "Option::is_none")]
    pub team_or_summoner_ids: Option<Vec<String>>,
}

impl SpectatorLcdsSpectateAvailabilityDto {
    pub fn new() -> SpectatorLcdsSpectateAvailabilityDto {
        SpectatorLcdsSpectateAvailabilityDto {
            team_or_summoner_ids: None,
        }
    }
}

