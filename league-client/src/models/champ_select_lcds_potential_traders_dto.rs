/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChampSelectLcdsPotentialTradersDto {
    #[serde(rename = "potentialTraders", skip_serializing_if = "Option::is_none")]
    pub potential_traders: Option<Vec<String>>,
}

impl ChampSelectLcdsPotentialTradersDto {
    pub fn new() -> ChampSelectLcdsPotentialTradersDto {
        ChampSelectLcdsPotentialTradersDto {
            potential_traders: None,
        }
    }
}

