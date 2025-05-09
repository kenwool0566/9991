use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct GameC2SSessionReq {
    pub timestamp: String,
    pub device_os_version: String,
    pub device_model: String,
    pub app_version: String,
    pub device_ids: Vec<DeviceId>,
    pub request_id: String,
    pub limit_ad_tracking: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct DeviceId {
    #[serde(rename = "type")]
    pub device_id_type: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct GamePatchVersionReq {
    // query
    pub version: String,
}
