#![allow(dead_code)]

use json_patch::merge as json_merge;

use reqwest::{Client, Response};
use serde_json::{json, Value};

mod input;
mod model;

mod util;

pub use input::get_files::{GetFiles, OrderBy, OrderDir};
pub use input::update_file::UpdateFile;
// pub use input::update_public::UpdatePublic;

pub use model::generic::*;
pub use model::get_files::*;

use util::deserialize;

const BASE_URL: &str = "https://uptobox.com/api/";

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
                serde_json::to_value(get_files).map_err(Error::ParseInput)?,
            )
            .await?;

        deserialize::<GetFilesResponseWrapper>(&response).map(|r| r.data)
    }

    /// Update file informations
    ///
    /// The following informations can be updated. Filename, Description, Password, Public
    pub async fn update_file(&self, update_file: &UpdateFile) -> UptoboxResult<usize> {
        let response = self
            .patch(
                "user/files",
                serde_json::to_value(update_file).map_err(Error::ParseInput)?,
            )
            .await?;

        dbg!(&response);

        deserialize::<GenericUpdatedResponseWrapper>(&response).map(|r| r.data.updated)
    }

    /// Not working
    pub async fn update_public(&self, file_codes: Vec<&str>, public: bool) -> UptoboxResult<usize> {
        let response = self
            .patch(
                "user/files",
                json!({ "file_codes": file_codes.join(","), "public": public }),
            )
            .await?;

        deserialize::<GenericUpdatedResponseWrapper>(&response).map(|r| r.data.updated)
    }

    /// Move a folder to another location
    pub async fn move_folder(
        &self,
        fld_id: usize,
        destination_fld_id: usize,
    ) -> UptoboxResult<String> {
        let response = self
            .patch(
                "user/files",
                json!({ "fld_id": fld_id, "destination_fld_id": destination_fld_id, "action": "move"}),
            )
            .await?;

        deserialize::<GenericMessageResponseWrapper>(&response).map(|r| {
            if r.status_code == 0 {
                Ok(r.data)
            } else {
                Err(Error::ParseResponse(
                    r.status_code,
                    r.data,
                    r.message.unwrap_or_default(),
                ))
            }
        })?
    }

    /// Move one or multiple files to another location
    pub async fn move_files(
        &self,
        file_codes: Vec<&str>,
        destination_fld_id: usize,
    ) -> UptoboxResult<usize> {
        let response = self
            .patch(
                "user/files",
                json!({ "file_codes": file_codes.join(","), "destination_fld_id": destination_fld_id, "action": "move"}),
            )
            .await?;

        deserialize::<GenericUpdatedResponseWrapper>(&response).map(|r| r.data.updated)
    }

    // TODO: "{\"success\":false,\"data\":\"Could not create alias : unknown file\"}
    /// Copy one or multiple files to another location
    pub async fn copy_files(
        &self,
        file_codes: Vec<&str>,
        destination_fld_id: usize,
    ) -> UptoboxResult<usize> {
        let response = self
            .patch(
                "user/files",
                json!({ "file_codes": file_codes.join(","), "destination_fld_id": destination_fld_id, "action": "copy"}),
            )
            .await?;

        deserialize::<GenericUpdatedResponseWrapper>(&response).map(|r| r.data.updated)
    }
}

impl Uptobox {
    pub fn new(key: &'static str) -> Self {
        Self {
            client: Client::new(),
            key,
        }
    }

    // /// Post a route
    // async fn post(&self, path: impl Into<String>, body: Value) -> UptoboxResult<String> {
    //     let body = self.add_token_auth(body);

    //     let res = self
    //         .client
    //         .post(format!("{BASE_URL}{}", path.into()))
    //         .json(&body)
    //         .send()
    //         .await
    //         .map_err(|e| Error::HttpRequest(e))?;

    //     self.parse_body(res).await
    // }

    /// Patch a route
    async fn patch(&self, path: impl Into<String>, body: Value) -> UptoboxResult<String> {
        let body = self.add_token_auth(body);

        let res = self
            .client
            .patch(format!("{BASE_URL}{}", path.into()))
            .json(&body)
            .send()
            .await
            .map_err(Error::HttpRequest)?;

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
            .map_err(Error::HttpRequest)?;

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
            Ok(res.text().await.map_err(Error::HttpRequest)?)
        } else {
            Err(Error::HttpResponseCode(res.status().as_u16()))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parse the input")]
    ParseInput(#[source] serde_json::Error),

    #[error("status_code: {0}, message: {1}, data: {2}")]
    ParseResponse(usize, String, String),

    #[error("Unable to parse the response with an unknown error")]
    UnknownParseResponse(#[source] serde_json::Error),

    #[error("Bad response")]
    HttpRequest(#[source] reqwest::Error),

    #[error("Got response {0}")]
    HttpResponseCode(u16),
}

pub type UptoboxResult<T = ()> = Result<T, Error>;
