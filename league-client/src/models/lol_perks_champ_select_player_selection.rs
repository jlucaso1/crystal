/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPerksChampSelectPlayerSelection {
    #[serde(rename = "assignedPosition", skip_serializing_if = "Option::is_none")]
    pub assigned_position: Option<String>,
    #[serde(rename = "cellId", skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<i64>,
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "championPickIntent", skip_serializing_if = "Option::is_none")]
    pub champion_pick_intent: Option<i32>,
    #[serde(rename = "selectedSkinId", skip_serializing_if = "Option::is_none")]
    pub selected_skin_id: Option<i32>,
    #[serde(rename = "spell1Id", skip_serializing_if = "Option::is_none")]
    pub spell1_id: Option<i64>,
    #[serde(rename = "spell2Id", skip_serializing_if = "Option::is_none")]
    pub spell2_id: Option<i64>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<i32>,
    #[serde(rename = "wardSkinId", skip_serializing_if = "Option::is_none")]
    pub ward_skin_id: Option<i64>,
}

impl LolPerksChampSelectPlayerSelection {
    pub fn new() -> LolPerksChampSelectPlayerSelection {
        LolPerksChampSelectPlayerSelection {
            assigned_position: None,
            cell_id: None,
            champion_id: None,
            champion_pick_intent: None,
            selected_skin_id: None,
            spell1_id: None,
            spell2_id: None,
            summoner_id: None,
            team: None,
            ward_skin_id: None,
        }
    }
}


