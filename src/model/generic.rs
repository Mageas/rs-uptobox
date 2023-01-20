use serde::Deserialize;

use crate::util::DeserializeCheck;

#[derive(Deserialize, Debug)]
pub struct GenericUpdatedResponseWrapper {
    pub status_code: usize,
    pub data: GenericUpdatedResponse,
    pub message: Option<String>,
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

impl DeserializeCheck for GenericMessageResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }

    fn data(&self) -> String {
        self.data.to_owned()
    }

    fn message(&self) -> Option<String> {
        self.message.to_owned()
    }
}

impl DeserializeCheck for GenericUpdatedResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }

    fn data(&self) -> String {
        String::new()
    }

    fn message(&self) -> Option<String> {
        self.message.to_owned()
    }
}
