/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolModeProgressionLoadoutsSlot {
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "inventoryType", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
}

impl LolModeProgressionLoadoutsSlot {
    pub fn new() -> LolModeProgressionLoadoutsSlot {
        LolModeProgressionLoadoutsSlot {
            content_id: None,
            inventory_type: None,
            item_id: None,
        }
    }
}

