#![allow(dead_code)]

use json_patch::merge as json_merge;

use reqwest::{Client, Response};
use serde_json::{json, Value};

mod input;
mod model;

mod util;

pub use input::get_files::{GetFiles, OrderBy, OrderDir};
pub use input::update_file::UpdateFile;

pub use model::generic::*;
pub use model::get_files::*;

const BASE_URL: &'static str = "https://uptobox.com/api/";

pub struct Uptobox {
    client: Client,
    key: &'static str,
}

impl Uptobox {
    /// Get files
    ///
    /// Retrieve files and folders
    pub async fn get_files(&self, get_files: &GetFiles) -> UptoboxResult<GetFilesResponse> {
        let response = self
            .get(
                "user/files",
                serde_json::to_value(get_files).map_err(|e| Error::ParseInput(e))?,
            )
            .await?;

        serde_json::from_str::<GetFilesResponseWrapper>(&response)
            .map(|r| r.data)
            .map_err(|e| Error::ParseResponse(e))
    }

    /// Update file informations
    ///
    /// The following informations can be updated. Filename, Description, Password, Public
    pub async fn update_file(&self, update_file: &UpdateFile) -> UptoboxResult<bool> {
        let response = self
            .patch(
                "user/files",
                serde_json::to_value(update_file).map_err(|e| Error::ParseInput(e))?,
            )
            .await?;

        serde_json::from_str::<GenericResponseWrapper>(&response)
            .map(|r| r.data.updated)
            .map_err(|e| Error::ParseResponse(e))
    }
}

impl Uptobox {
    pub fn new(key: &'static str) -> Self {
        Self {
            client: Client::new(),
            key,
        }
    }

    /// Post a route
    async fn post(&self, path: impl Into<String>, body: Value) -> UptoboxResult<String> {
        let body = self.add_token_auth(body);

        let res = self
            .client
            .post(format!("{BASE_URL}{}", path.into()))
            .form(&body)
            .send()
            .await
            .map_err(|e| Error::HttpRequest(e))?;

        self.parse_body(res).await
    }

    /// Patch a route
    async fn patch(&self, path: impl Into<String>, body: Value) -> UptoboxResult<String> {
        let body = self.add_token_auth(body);

        let res = self
            .client
            .patch(format!("{BASE_URL}{}", path.into()))
            .form(&body)
            .send()
            .await
            .map_err(|e| Error::HttpRequest(e))?;

        self.parse_body(res).await
    }

    /// Get a public route
    async fn public_get(&self, path: impl Into<String>, params: Value) -> UptoboxResult<String> {
        let res = self
            .client
            .get(format!("{BASE_URL}{}", path.into()))
            .query(&params)
            .send()
            .await
            .map_err(|e| Error::HttpRequest(e))?;

        self.parse_body(res).await
    }

    /// Get a private route
    async fn get(&self, path: impl Into<String>, params: Value) -> UptoboxResult<String> {
        let params = self.add_token_auth(params);
        self.public_get(path, params).await
    }

    /// Add the auth token to the params or the body
    fn add_token_auth(&self, input: Value) -> Value {
        let mut secret = json!({ "token": self.key});
        json_merge(&mut secret, &input);
        secret
    }

    /// Parse the body of a response as json, and check if the response status is 200.
    /// If the response status is 200, it returns the json value, otherwise it returns an Error::HttpResponseCode variant with the status code.
    async fn parse_body(&self, res: Response) -> UptoboxResult<String> {
        if res.status() == 200 {
            Ok(res.text().await.map_err(|e| Error::HttpRequest(e))?)
        } else {
            Err(Error::HttpResponseCode(res.status().as_u16()))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parse the input")]
    ParseInput(#[source] serde_json::Error),

    #[error("Parse the response")]
    ParseResponse(#[source] serde_json::Error),

    #[error("Bad response")]
    HttpRequest(#[source] reqwest::Error),

    #[error("Got response {0}")]
    HttpResponseCode(u16),
}

pub type UptoboxResult<T = ()> = Result<T, Error>;
