/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPlayerMessagingDynamicCelebrationMessagingNotificationResource {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "celebrationBody", skip_serializing_if = "Option::is_none")]
    pub celebration_body: Option<String>,
    #[serde(rename = "celebrationMessage", skip_serializing_if = "Option::is_none")]
    pub celebration_message: Option<String>,
    #[serde(rename = "celebrationTitle", skip_serializing_if = "Option::is_none")]
    pub celebration_title: Option<String>,
    #[serde(rename = "celebrationType", skip_serializing_if = "Option::is_none")]
    pub celebration_type: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "inventoryType", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "itemQuantity", skip_serializing_if = "Option::is_none")]
    pub item_quantity: Option<String>,
    #[serde(rename = "msgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl LolPlayerMessagingDynamicCelebrationMessagingNotificationResource {
    pub fn new() -> LolPlayerMessagingDynamicCelebrationMessagingNotificationResource {
        LolPlayerMessagingDynamicCelebrationMessagingNotificationResource {
            account_id: None,
            celebration_body: None,
            celebration_message: None,
            celebration_title: None,
            celebration_type: None,
            id: None,
            inventory_type: None,
            item_id: None,
            item_quantity: None,
            msg_id: None,
            status: None,
        }
    }
}

