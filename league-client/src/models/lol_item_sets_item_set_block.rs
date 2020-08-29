/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolItemSetsItemSetBlock {
    #[serde(rename = "hideIfSummonerSpell", skip_serializing_if = "Option::is_none")]
    pub hide_if_summoner_spell: Option<String>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::LolItemSetsItemSetItem>>,
    #[serde(rename = "showIfSummonerSpell", skip_serializing_if = "Option::is_none")]
    pub show_if_summoner_spell: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LolItemSetsItemSetBlock {
    pub fn new() -> LolItemSetsItemSetBlock {
        LolItemSetsItemSetBlock {
            hide_if_summoner_spell: None,
            items: None,
            show_if_summoner_spell: None,
            _type: None,
        }
    }
}


