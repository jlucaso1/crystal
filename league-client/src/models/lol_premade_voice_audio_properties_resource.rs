/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolPremadeVoiceAudioPropertiesResource {
    #[serde(rename = "isLoopbackEnabled", skip_serializing_if = "Option::is_none")]
    pub is_loopback_enabled: Option<bool>,
    #[serde(rename = "micEnergy", skip_serializing_if = "Option::is_none")]
    pub mic_energy: Option<i32>,
}

impl LolPremadeVoiceAudioPropertiesResource {
    pub fn new() -> LolPremadeVoiceAudioPropertiesResource {
        LolPremadeVoiceAudioPropertiesResource {
            is_loopback_enabled: None,
            mic_energy: None,
        }
    }
}


