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
}

impl DeserializeCheck for GetAccountPaymentsResponseWrapper {
    fn status_code(&self) -> usize {
        self.status_code
    }
}
