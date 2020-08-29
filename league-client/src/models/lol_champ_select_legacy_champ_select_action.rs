/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChampSelectLegacyChampSelectAction {
    #[serde(rename = "actorCellId", skip_serializing_if = "Option::is_none")]
    pub actor_cell_id: Option<i64>,
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isAllyAction", skip_serializing_if = "Option::is_none")]
    pub is_ally_action: Option<bool>,
    #[serde(rename = "isInProgress", skip_serializing_if = "Option::is_none")]
    pub is_in_progress: Option<bool>,
    #[serde(rename = "pickTurn", skip_serializing_if = "Option::is_none")]
    pub pick_turn: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LolChampSelectLegacyChampSelectAction {
    pub fn new() -> LolChampSelectLegacyChampSelectAction {
        LolChampSelectLegacyChampSelectAction {
            actor_cell_id: None,
            champion_id: None,
            completed: None,
            id: None,
            is_ally_action: None,
            is_in_progress: None,
            pick_turn: None,
            _type: None,
        }
    }
}


