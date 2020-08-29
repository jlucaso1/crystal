/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MissionDisplay {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
}

impl MissionDisplay {
    pub fn new() -> MissionDisplay {
        MissionDisplay {
            attributes: None,
            locations: None,
        }
    }
}

