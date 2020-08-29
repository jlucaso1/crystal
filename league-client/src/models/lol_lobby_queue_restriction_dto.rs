/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyQueueRestrictionDto {
    #[serde(rename = "gatekeeperRestrictions", skip_serializing_if = "Option::is_none")]
    pub gatekeeper_restrictions: Option<Vec<crate::models::LolLobbyGatekeeperRestrictionDto>>,
}

impl LolLobbyQueueRestrictionDto {
    pub fn new() -> LolLobbyQueueRestrictionDto {
        LolLobbyQueueRestrictionDto {
            gatekeeper_restrictions: None,
        }
    }
}

