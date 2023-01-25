use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesFromPublicFolder {
    // The folder id
    folder: usize,

    // The folder hash
    hash: String,

    // Number of files to retrieve
    limit: usize,

    // Retrieve from the specified offset
    offset: usize,
}

impl GetFilesFromPublicFolder {
    /// Create a new GetFiles
    pub fn new(folder: usize, hash: impl Into<String>) -> Self {
        Self {
            folder,
            hash: hash.into(),
            ..Default::default()
        }
    }

    // Set the limit of files to retrieve
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    // Set the offset of the directory
    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        self
    }
}

impl Default for GetFilesFromPublicFolder {
    fn default() -> Self {
        Self {
            folder: 0,
            hash: "".into(),
            limit: 100,
            offset: 0,
        }
    }
}
