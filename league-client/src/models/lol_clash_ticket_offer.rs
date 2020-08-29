/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolClashTicketOffer {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "isAccepted", skip_serializing_if = "Option::is_none")]
    pub is_accepted: Option<bool>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "ticketType", skip_serializing_if = "Option::is_none")]
    pub ticket_type: Option<crate::models::TicketType>,
}

impl LolClashTicketOffer {
    pub fn new() -> LolClashTicketOffer {
        LolClashTicketOffer {
            amount: None,
            is_accepted: None,
            summoner_id: None,
            ticket_type: None,
        }
    }
}


