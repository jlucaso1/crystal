/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolInventoryRmsEntitlementPayload {
    #[serde(rename = "entitlementTypeId", skip_serializing_if = "Option::is_none")]
    pub entitlement_type_id: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "itemTypeId", skip_serializing_if = "Option::is_none")]
    pub item_type_id: Option<String>,
    #[serde(rename = "resourceOperation", skip_serializing_if = "Option::is_none")]
    pub resource_operation: Option<String>,
}

impl LolInventoryRmsEntitlementPayload {
    pub fn new() -> LolInventoryRmsEntitlementPayload {
        LolInventoryRmsEntitlementPayload {
            entitlement_type_id: None,
            item_id: None,
            item_type_id: None,
            resource_operation: None,
        }
    }
}


