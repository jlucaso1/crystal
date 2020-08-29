/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPftPftEndOfGamePoints {
    #[serde(rename = "pointChangeFromChampionsOwned", skip_serializing_if = "Option::is_none")]
    pub point_change_from_champions_owned: Option<i32>,
    #[serde(rename = "pointChangeFromGameplay", skip_serializing_if = "Option::is_none")]
    pub point_change_from_gameplay: Option<i32>,
    #[serde(rename = "pointsUntilNextReroll", skip_serializing_if = "Option::is_none")]
    pub points_until_next_reroll: Option<i32>,
    #[serde(rename = "pointsUsed", skip_serializing_if = "Option::is_none")]
    pub points_used: Option<i32>,
    #[serde(rename = "previousPoints", skip_serializing_if = "Option::is_none")]
    pub previous_points: Option<i32>,
    #[serde(rename = "rerollCount", skip_serializing_if = "Option::is_none")]
    pub reroll_count: Option<i32>,
    #[serde(rename = "totalPoints", skip_serializing_if = "Option::is_none")]
    pub total_points: Option<i32>,
}

impl LolPftPftEndOfGamePoints {
    pub fn new() -> LolPftPftEndOfGamePoints {
        LolPftPftEndOfGamePoints {
            point_change_from_champions_owned: None,
            point_change_from_gameplay: None,
            points_until_next_reroll: None,
            points_used: None,
            previous_points: None,
            reroll_count: None,
            total_points: None,
        }
    }
}


