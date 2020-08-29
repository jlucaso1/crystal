/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPerksChampSelectAction {
    #[serde(rename = "actorCellId", skip_serializing_if = "Option::is_none")]
    pub actor_cell_id: Option<i64>,
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LolPerksChampSelectAction {
    pub fn new() -> LolPerksChampSelectAction {
        LolPerksChampSelectAction {
            actor_cell_id: None,
            champion_id: None,
            completed: None,
            id: None,
            _type: None,
        }
    }
}

