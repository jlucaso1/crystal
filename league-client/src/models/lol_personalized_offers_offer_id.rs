/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPersonalizedOffersOfferId {
    #[serde(rename = "offerId", skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
}

impl LolPersonalizedOffersOfferId {
    pub fn new() -> LolPersonalizedOffersOfferId {
        LolPersonalizedOffersOfferId {
            offer_id: None,
        }
    }
}


