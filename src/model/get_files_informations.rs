use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetFilesInformationsResponseWrapper {
    pub status_code: usize,
    pub data: GetFilesInformationsListResponse,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GetFilesInformationsListResponse {
    pub list: Vec<GetFilesInformationsResponse>,
}

#[derive(Deserialize, Debug)]
pub struct GetFilesInformationsResponse {
    pub file_code: String,
    pub file_name: String,
    pub file_size: usize,
    pub available_uts: bool,
    pub need_premium: bool,
    pub error: Option<GetFilesInformationsError>,
}

#[derive(Deserialize, Debug)]
pub struct GetFilesInformationsError {
    pub code: usize,
    pub message: String,
}

impl DeserializeCheck for GetFilesInformationsResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}
