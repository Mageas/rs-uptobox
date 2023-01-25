use serde::Deserialize;

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetDownloadUrlResponseWrapper {
    pub status_code: usize,
    pub data: GetDownloadUrlResponse,
    pub message: Option<String>,
}

/// Response
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetDownloadUrlResponse {
    Link(GetDownloadUrlLink),
    Wait(GetDownloadUrlWait),
}

/// Data of GetDownloadUrlResponse
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadUrlLink {
    pub dl_link: String,
}

/// Data of GetDownloadUrlResponse
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDownloadUrlWait {
    pub waiting: usize,
    pub waiting_token: Option<String>,
}

impl DeserializeCheck for GetDownloadUrlResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}
