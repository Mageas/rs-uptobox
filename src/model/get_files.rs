use serde::Deserialize;

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetFilesResponseWrapper {
    pub status_code: usize,
    pub data: GetFilesResponse,
    pub message: Option<String>,
}

/// Response
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesResponse {
    pub current_folder: GetFilesCurrentFolder,
    pub folders: Vec<GetFilesFolders>,
    pub files: Vec<GetFilesFiles>,
    pub page_count: usize,
    pub total_file_count: usize,
    pub total_file_size: usize,
}

/// Data of GetFilesResponse
#[derive(Deserialize, Debug)]
pub struct GetFilesCurrentFolder {
    #[serde(rename = "fileCount")]
    pub file_count: usize,
    pub fld_id: usize,
    #[serde(default = "default_string")]
    pub fld_name: String,
    pub fld_parent_id: Option<usize>,
    pub hash: String,
    #[serde(default = "default_string")]
    pub name: String,
    #[serde(rename = "totalFileSize")]
    pub total_file_size: usize,
}

/// Data of GetFilesResponse
#[derive(Deserialize, Debug)]
pub struct GetFilesFolders {
    pub fld_id: usize,
    pub fld_name: String,
    pub hash: String,
    pub name: String,
}

/// Data of GetFilesResponse
#[derive(Deserialize, Debug)]
pub struct GetFilesFiles {
    pub file_code: String,
    pub file_created: String,
    pub file_descr: String,
    pub file_downloads: usize,
    pub file_last_download: String,
    pub file_name: String,
    pub file_password: String,
    pub file_public: usize,
    pub file_size: usize,
    pub id: Option<usize>,
    pub last_stream: String,
    pub nb_stream: usize,
    pub transcoded: Option<usize>,
}

fn default_string() -> String {
    "//".to_string()
}

impl DeserializeCheck for GetFilesResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}
