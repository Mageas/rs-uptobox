use std::fmt;

use serde::Serialize;

#[derive(Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct GetDownloadUrl {
    // The file code
    file_code: String,

    // The password
    password: String,
}

impl GetDownloadUrl {
    /// Create a new GetFiles
    pub fn new(file_code: impl Into<String>) -> Self {
        Self {
            file_code: file_code.into(),
            ..Default::default()
        }
    }

    // Set the limit of files to retrieve
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        self.password = password.into();
        self
    }
}

impl Default for GetDownloadUrl {
    fn default() -> Self {
        Self {
            file_code: String::new(),
            password: String::new(),
        }
    }
}
