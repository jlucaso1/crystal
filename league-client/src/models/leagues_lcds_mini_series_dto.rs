/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeaguesLcdsMiniSeriesDto {
    #[serde(rename = "losses", skip_serializing_if = "Option::is_none")]
    pub losses: Option<i64>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<i64>,
    #[serde(rename = "timeLeftToPlayMillis", skip_serializing_if = "Option::is_none")]
    pub time_left_to_play_millis: Option<i64>,
    #[serde(rename = "wins", skip_serializing_if = "Option::is_none")]
    pub wins: Option<i64>,
}

impl LeaguesLcdsMiniSeriesDto {
    pub fn new() -> LeaguesLcdsMiniSeriesDto {
        LeaguesLcdsMiniSeriesDto {
            losses: None,
            progress: None,
            target: None,
            time_left_to_play_millis: None,
            wins: None,
        }
    }
}

