use serde::Serialize;

/// Input
#[derive(Serialize, Debug)]
pub struct UpdateFile {
    /// The file code
    file_code: String,

    /// New file name value
    #[serde(skip_serializing_if = "Option::is_none")]
    new_name: Option<String>,

    /// New description vale
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// New password value
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,

    /// New public status
    #[serde(skip_serializing_if = "Option::is_none")]
    public: Option<bool>,
}

impl UpdateFile {
    /// Create a new instance
    ///
    /// **1 modifier is requiered for the request to succeed (eg. name, description)**
    pub fn new(file_code: impl Into<String>) -> Self {
        Self {
            file_code: file_code.into(),
            ..Default::default()
        }
    }

    /// Update the file name
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.new_name.insert(name.into());
        self
    }

    /// Udate the file description
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }

    /// Update the file password
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        let _ = self.password.insert(password.into());
        self
    }

    /// Update the file public status
    pub fn public(&mut self, public: bool) -> &mut Self {
        let _ = self.public.insert(public);
        self
    }
}

impl Default for UpdateFile {
    fn default() -> Self {
        Self {
            file_code: "".into(),
            new_name: None,
            description: None,
            password: None,
            public: None,
        }
    }
}
