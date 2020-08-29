/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolLootRecipeOutput {
    #[serde(rename = "lootName", skip_serializing_if = "Option::is_none")]
    pub loot_name: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl LolLootRecipeOutput {
    pub fn new() -> LolLootRecipeOutput {
        LolLootRecipeOutput {
            loot_name: None,
            quantity: None,
        }
    }
}

