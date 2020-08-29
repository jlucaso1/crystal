/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCosmeticsGameDataTftMapSkin {
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "loadoutsIcon", skip_serializing_if = "Option::is_none")]
    pub loadouts_icon: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "rarityValue", skip_serializing_if = "Option::is_none")]
    pub rarity_value: Option<i32>,
}

impl LolCosmeticsGameDataTftMapSkin {
    pub fn new() -> LolCosmeticsGameDataTftMapSkin {
        LolCosmeticsGameDataTftMapSkin {
            content_id: None,
            description: None,
            group_id: None,
            group_name: None,
            item_id: None,
            loadouts_icon: None,
            name: None,
            rarity_value: None,
        }
    }
}

