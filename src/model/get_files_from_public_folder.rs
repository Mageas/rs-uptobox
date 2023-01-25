use serde::Deserialize;

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetFilesFromPublicFolderResponseWrapper {
    pub status_code: usize,
    pub data: GetFilesFromPublicFolderListResponse,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetFilesFromPublicFolderListResponse {
    pub list: Vec<GetFilesFromPublicFolderResponse>,
}

#[derive(Deserialize, Debug)]
pub struct GetFilesFromPublicFolderResponse {
    pub file_name: String,
    pub file_date_inserted: String,
    pub file_code: String,
}

impl DeserializeCheck for GetFilesFromPublicFolderResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}
