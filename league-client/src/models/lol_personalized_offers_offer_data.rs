/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPersonalizedOffersOfferData {
    #[serde(rename = "offers", skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<crate::models::LolPersonalizedOffersOffer>>,
    #[serde(rename = "promotion", skip_serializing_if = "Option::is_none")]
    pub promotion: Option<crate::models::LolPersonalizedOffersPromotion>,
}

impl LolPersonalizedOffersOfferData {
    pub fn new() -> LolPersonalizedOffersOfferData {
        LolPersonalizedOffersOfferData {
            offers: None,
            promotion: None,
        }
    }
}


