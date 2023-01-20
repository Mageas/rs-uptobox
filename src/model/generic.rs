use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GenericUpdatedResponseWrapper {
    pub data: GenericUpdatedResponse,
}

#[derive(Deserialize, Debug)]
pub struct GenericUpdatedResponse {
    pub updated: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenericMessageResponseWrapper {
    pub status_code: usize,
    pub data: String,
    pub message: Option<String>,
}
