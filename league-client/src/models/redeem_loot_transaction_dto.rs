/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedeemLootTransactionDto {
    #[serde(rename = "lootName", skip_serializing_if = "Option::is_none")]
    pub loot_name: Option<String>,
}

impl RedeemLootTransactionDto {
    pub fn new() -> RedeemLootTransactionDto {
        RedeemLootTransactionDto {
            loot_name: None,
        }
    }
}

