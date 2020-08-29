/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPftPftQuestionResponse {
    #[serde(rename = "questionId", skip_serializing_if = "Option::is_none")]
    pub question_id: Option<i64>,
    #[serde(rename = "responseData", skip_serializing_if = "Option::is_none")]
    pub response_data: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl LolPftPftQuestionResponse {
    pub fn new() -> LolPftPftQuestionResponse {
        LolPftPftQuestionResponse {
            question_id: None,
            response_data: None,
        }
    }
}

