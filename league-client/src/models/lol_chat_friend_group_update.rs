/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatFriendGroupUpdate {
    #[serde(rename = "collapsed", skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "new_name", skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}

impl LolChatFriendGroupUpdate {
    pub fn new() -> LolChatFriendGroupUpdate {
        LolChatFriendGroupUpdate {
            collapsed: None,
            name: None,
            new_name: None,
        }
    }
}


