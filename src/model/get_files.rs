use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetFilesResponseWrapper {
    pub data: GetFilesResponse,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesResponse {
    current_folder: GetFilesCurrentFolder,
    folders: Vec<GetFilesFolders>,
    files: Vec<GetFilesFiles>,
    page_count: usize,
    total_file_count: usize,
    total_file_size: usize,
}

#[derive(Deserialize, Debug)]
pub struct GetFilesCurrentFolder {
    #[serde(rename = "fileCount")]
    file_count: usize,
    fld_id: usize,
    #[serde(default = "default_string")]
    fld_name: String,
    fld_parent_id: Option<usize>,
    hash: String,
    #[serde(default = "default_string")]
    name: String,
    #[serde(rename = "totalFileSize")]
    total_file_size: usize,
}

#[derive(Deserialize, Debug)]
pub struct GetFilesFolders {
    fld_id: usize,
    fld_name: String,
    hash: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct GetFilesFiles {
    file_code: String,
    file_created: String,
    file_descr: String,
    file_downloads: usize,
    file_last_download: String,
    file_name: String,
    file_password: String,
    file_public: usize,
    file_size: usize,
    id: Option<usize>,
    last_stream: String,
    nb_stream: usize,
    transcoded: Option<usize>,
}

fn default_string() -> String {
    "//".to_string()
}
