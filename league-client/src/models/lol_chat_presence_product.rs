/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatPresenceProduct {
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

impl LolChatPresenceProduct {
    pub fn new() -> LolChatPresenceProduct {
        LolChatPresenceProduct {
            product: None,
        }
    }
}


