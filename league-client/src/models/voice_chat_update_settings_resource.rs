/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceChatUpdateSettingsResource {
    #[serde(rename = "currentCaptureDeviceHandle", skip_serializing_if = "Option::is_none")]
    pub current_capture_device_handle: Option<String>,
    #[serde(rename = "currentRenderDeviceHandle", skip_serializing_if = "Option::is_none")]
    pub current_render_device_handle: Option<String>,
    #[serde(rename = "localMicMuted", skip_serializing_if = "Option::is_none")]
    pub local_mic_muted: Option<bool>,
    #[serde(rename = "loopbackEnabled", skip_serializing_if = "Option::is_none")]
    pub loopback_enabled: Option<bool>,
    #[serde(rename = "micLevel", skip_serializing_if = "Option::is_none")]
    pub mic_level: Option<i32>,
    #[serde(rename = "speakerLevel", skip_serializing_if = "Option::is_none")]
    pub speaker_level: Option<i32>,
    #[serde(rename = "vadHangoverTime", skip_serializing_if = "Option::is_none")]
    pub vad_hangover_time: Option<i32>,
    #[serde(rename = "vadSensitivity", skip_serializing_if = "Option::is_none")]
    pub vad_sensitivity: Option<i32>,
}

impl VoiceChatUpdateSettingsResource {
    pub fn new() -> VoiceChatUpdateSettingsResource {
        VoiceChatUpdateSettingsResource {
            current_capture_device_handle: None,
            current_render_device_handle: None,
            local_mic_muted: None,
            loopback_enabled: None,
            mic_level: None,
            speaker_level: None,
            vad_hangover_time: None,
            vad_sensitivity: None,
        }
    }
}


