/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResultDto {
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<i64>,
    #[serde(rename = "queryToLootNames", skip_serializing_if = "Option::is_none")]
    pub query_to_loot_names: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl QueryResultDto {
    pub fn new() -> QueryResultDto {
        QueryResultDto {
            last_update: None,
            query_to_loot_names: None,
        }
    }
}

