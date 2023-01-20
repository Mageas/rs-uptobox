use std::fmt;

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFiles {
    // The folder path
    path: String,

    // Number of files to retrieve
    limit: usize,

    // Sort the result by a column name
    order_by: OrderBy,

    // Sort direction if orderBy is provided (ASC , DESC ) default : 'ASC'
    dir: OrderDir,

    // Retrieve from the specified offset
    offset: usize,

    // The search field column name
    #[serde(skip_serializing_if = "Option::is_none")]
    search_field: Option<OrderBy>,

    // Search content
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
}

/// Column of the order
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    FileName,
    FileDate,
    FileSize,
    Transcoded,
    FileDownloads,
}

/// Direction of the order
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderDir {
    Asc,
    Desc,
}

impl fmt::Display for OrderDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderDir::Asc => write!(f, "ASC"),
            OrderDir::Desc => write!(f, "DESC"),
        }
    }
}

impl GetFiles {
    /// Create a new GetFiles
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
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

    // Set the order column of the directory
    pub fn order_by(&mut self, order_by: OrderBy) -> &mut Self {
        self.order_by = order_by;
        self
    }

    // Set the order direction of the directory
    pub fn order_dir(&mut self, order_dir: OrderDir) -> &mut Self {
        self.dir = order_dir;
        self
    }

    // Set the search
    pub fn search(&mut self, search_field: OrderBy, search: impl Into<String>) -> &mut Self {
        let _ = self.search_field.insert(search_field);
        let _ = self.search.insert(search.into());
        self
    }
}

impl Default for GetFiles {
    fn default() -> Self {
        Self {
            path: "//".into(),
            limit: 100,
            offset: 0,
            order_by: OrderBy::FileName,
            dir: OrderDir::Asc,
            search_field: None,
            search: None,
        }
    }
}
