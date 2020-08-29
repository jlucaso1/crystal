/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatFriendResource {
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
    #[serde(rename = "displayGroupId", skip_serializing_if = "Option::is_none")]
    pub display_group_id: Option<i32>,
    #[serde(rename = "displayGroupName", skip_serializing_if = "Option::is_none")]
    pub display_group_name: Option<String>,
    #[serde(rename = "gameName", skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    #[serde(rename = "gameTag", skip_serializing_if = "Option::is_none")]
    pub game_tag: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isP2PConversationMuted", skip_serializing_if = "Option::is_none")]
    pub is_p2_p_conversation_muted: Option<bool>,
    #[serde(rename = "lastSeenOnlineTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_seen_online_timestamp: Option<String>,
    #[serde(rename = "lol", skip_serializing_if = "Option::is_none")]
    pub lol: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "patchline", skip_serializing_if = "Option::is_none")]
    pub patchline: Option<String>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(rename = "platformId", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "productName", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "statusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

impl LolChatFriendResource {
    pub fn new() -> LolChatFriendResource {
        LolChatFriendResource {
            availability: None,
            display_group_id: None,
            display_group_name: None,
            game_name: None,
            game_tag: None,
            group_id: None,
            group_name: None,
            icon: None,
            id: None,
            is_p2_p_conversation_muted: None,
            last_seen_online_timestamp: None,
            lol: None,
            name: None,
            note: None,
            patchline: None,
            pid: None,
            platform_id: None,
            product: None,
            product_name: None,
            puuid: None,
            status_message: None,
            summary: None,
            summoner_id: None,
            time: None,
        }
    }
}


