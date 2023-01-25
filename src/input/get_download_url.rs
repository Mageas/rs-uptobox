use serde::Serialize;

/// Input
#[derive(Serialize, Default, Debug)]
pub struct GetDownloadUrl {
    /// The file code
    file_code: String,

    /// The password
    password: String,
}

impl GetDownloadUrl {
    /// Create a new instance
    pub fn new(file_code: impl Into<String>) -> Self {
        Self {
            file_code: file_code.into(),
            ..Default::default()
        }
    }

    /// Set the password
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        self.password = password.into();
        self
    }
}
