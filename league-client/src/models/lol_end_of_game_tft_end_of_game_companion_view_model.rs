/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolEndOfGameTftEndOfGameCompanionViewModel {
    #[serde(rename = "colorName", skip_serializing_if = "Option::is_none")]
    pub color_name: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "speciesName", skip_serializing_if = "Option::is_none")]
    pub species_name: Option<String>,
}

impl LolEndOfGameTftEndOfGameCompanionViewModel {
    pub fn new() -> LolEndOfGameTftEndOfGameCompanionViewModel {
        LolEndOfGameTftEndOfGameCompanionViewModel {
            color_name: None,
            icon: None,
            species_name: None,
        }
    }
}

