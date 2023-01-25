use serde::Deserialize;

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetUploadUrlResponseWrapper {
    pub status_code: usize,
    pub data: GetUploadUrlResponse,
    pub message: Option<String>,
}

/// Response
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetUploadUrlResponse {
    pub upload_link: String,
    pub max_upload: String,
}

impl DeserializeCheck for GetUploadUrlResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}
