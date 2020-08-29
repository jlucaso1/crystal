/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolHonorV2Honor {
    #[serde(rename = "honorCategory", skip_serializing_if = "Option::is_none")]
    pub honor_category: Option<String>,
    #[serde(rename = "voterRelationship", skip_serializing_if = "Option::is_none")]
    pub voter_relationship: Option<String>,
}

impl LolHonorV2Honor {
    pub fn new() -> LolHonorV2Honor {
        LolHonorV2Honor {
            honor_category: None,
            voter_relationship: None,
        }
    }
}


