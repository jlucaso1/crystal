/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatChatFriendUpdate {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
}

impl LolChatChatFriendUpdate {
    pub fn new() -> LolChatChatFriendUpdate {
        LolChatChatFriendUpdate {
            group: None,
            note: None,
            pid: None,
        }
    }
}


