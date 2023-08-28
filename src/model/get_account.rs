use serde::Deserialize;

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetAccountResponseWrapper {
    pub status_code: usize,
    pub data: GetAccountResponse,
    pub message: Option<String>,
}

/// Response
#[derive(Deserialize, Debug)]
pub struct GetAccountResponse {
    #[serde(deserialize_with = "deserialize_usize_to_bool")]
    pub premium: bool,
    pub login: String,
    pub email: String,
    pub point: f32,
    pub premium_expire: String,
    #[serde(deserialize_with = "deserialize_usize_to_bool")]
    #[serde(rename = "securityLock")]
    pub security_lock: bool,
    #[serde(deserialize_with = "deserialize_usize_to_bool")]
    #[serde(rename = "directDownload")]
    pub direct_download: bool,
    #[serde(deserialize_with = "deserialize_usize_to_bool")]
    #[serde(rename = "sslDownload")]
    pub ssl_download: bool,
    pub token: String,
}

impl DeserializeCheck for GetAccountResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}

fn deserialize_usize_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let usize_value = usize::deserialize(deserializer)?;
    Ok(usize_value != 0)
}
