use serde::Deserialize;

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
pub(crate) struct GenericUpdatedResponseWrapper {
    #[serde(alias = "statusCode")]
    pub status_code: usize,
    pub data: GenericUpdatedResponse,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GenericUpdatedResponse {
    #[serde(alias = "deleted")]
    pub updated: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenericMessageResponseWrapper {
    pub status_code: usize,
    pub data: String,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenericEmpyDataResponseWrapper {
    pub status_code: usize,
    pub message: Option<String>,
}

impl DeserializeCheck for GenericMessageResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}

impl DeserializeCheck for GenericUpdatedResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}

impl DeserializeCheck for GenericEmpyDataResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}
