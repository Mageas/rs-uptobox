use serde::Deserialize;

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetAccountPaymentsResponseWrapper {
    pub status_code: usize,
    pub data: GetAccountPaymentsListResponseWrapper,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetAccountPaymentsListResponseWrapper {
    pub list: Vec<GetAccountPaymentsResponse>,
}

#[derive(Deserialize, Debug)]
pub struct GetAccountPaymentsResponse {
    created: String,
    status: String,
    amount: String,
    days: usize,
    #[serde(rename = "type")]
    _type: String,
    // #[serde(deserialize_with = "deserialize_usize_to_bool")]
    // pub premium: bool,
    // pub login: String,
    // pub email: String,
    // pub point: String,
    // pub premium_expire: String,
    // #[serde(deserialize_with = "deserialize_usize_to_bool")]
    // #[serde(rename = "securityLock")]
    // pub security_lock: bool,
    // #[serde(deserialize_with = "deserialize_usize_to_bool")]
    // #[serde(rename = "directDownload")]
    // pub direct_download: bool,
    // #[serde(deserialize_with = "deserialize_usize_to_bool")]
    // #[serde(rename = "sslDownload")]
    // pub ssl_download: bool,
    // pub token: String,
}

impl DeserializeCheck for GetAccountPaymentsResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}