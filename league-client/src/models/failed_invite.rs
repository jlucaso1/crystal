/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedInvite {
    #[serde(rename = "exception", skip_serializing_if = "Option::is_none")]
    pub exception: Option<crate::models::ClientRequestError>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
}

impl FailedInvite {
    pub fn new() -> FailedInvite {
        FailedInvite {
            exception: None,
            player_id: None,
        }
    }
}


