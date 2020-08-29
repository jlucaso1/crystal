/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolCollectionsNumberFormattingData {
    #[serde(rename = "billionAbbreviation", skip_serializing_if = "Option::is_none")]
    pub billion_abbreviation: Option<String>,
    #[serde(rename = "decimalSeperator", skip_serializing_if = "Option::is_none")]
    pub decimal_seperator: Option<String>,
    #[serde(rename = "hourAbbreviation", skip_serializing_if = "Option::is_none")]
    pub hour_abbreviation: Option<String>,
    #[serde(rename = "kilometersAbbreviation", skip_serializing_if = "Option::is_none")]
    pub kilometers_abbreviation: Option<String>,
    #[serde(rename = "metersAbbreviation", skip_serializing_if = "Option::is_none")]
    pub meters_abbreviation: Option<String>,
    #[serde(rename = "millionAbbreviation", skip_serializing_if = "Option::is_none")]
    pub million_abbreviation: Option<String>,
    #[serde(rename = "minuteAbbreviation", skip_serializing_if = "Option::is_none")]
    pub minute_abbreviation: Option<String>,
    #[serde(rename = "numberFormattingBehavior", skip_serializing_if = "Option::is_none")]
    pub number_formatting_behavior: Option<crate::models::LolCollectionsNumberFormattingBehavior>,
    #[serde(rename = "oneHundredMillionAbbreviation", skip_serializing_if = "Option::is_none")]
    pub one_hundred_million_abbreviation: Option<String>,
    #[serde(rename = "percentageFormat", skip_serializing_if = "Option::is_none")]
    pub percentage_format: Option<String>,
    #[serde(rename = "secondAbbreviation", skip_serializing_if = "Option::is_none")]
    pub second_abbreviation: Option<String>,
    #[serde(rename = "tenThousandAbbreviation", skip_serializing_if = "Option::is_none")]
    pub ten_thousand_abbreviation: Option<String>,
    #[serde(rename = "thousandAbbreviation", skip_serializing_if = "Option::is_none")]
    pub thousand_abbreviation: Option<String>,
    #[serde(rename = "thousandSeperator", skip_serializing_if = "Option::is_none")]
    pub thousand_seperator: Option<String>,
    #[serde(rename = "trillionAbbreviation", skip_serializing_if = "Option::is_none")]
    pub trillion_abbreviation: Option<String>,
}

impl LolCollectionsNumberFormattingData {
    pub fn new() -> LolCollectionsNumberFormattingData {
        LolCollectionsNumberFormattingData {
            billion_abbreviation: None,
            decimal_seperator: None,
            hour_abbreviation: None,
            kilometers_abbreviation: None,
            meters_abbreviation: None,
            million_abbreviation: None,
            minute_abbreviation: None,
            number_formatting_behavior: None,
            one_hundred_million_abbreviation: None,
            percentage_format: None,
            second_abbreviation: None,
            ten_thousand_abbreviation: None,
            thousand_abbreviation: None,
            thousand_seperator: None,
            trillion_abbreviation: None,
        }
    }
}


