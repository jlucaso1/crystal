/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPerksChampSelectChatRoomDetails {
    #[serde(rename = "chatRoomName", skip_serializing_if = "Option::is_none")]
    pub chat_room_name: Option<String>,
    #[serde(rename = "chatRoomPassword", skip_serializing_if = "Option::is_none")]
    pub chat_room_password: Option<String>,
}

impl LolPerksChampSelectChatRoomDetails {
    pub fn new() -> LolPerksChampSelectChatRoomDetails {
        LolPerksChampSelectChatRoomDetails {
            chat_room_name: None,
            chat_room_password: None,
        }
    }
}


