#![allow(dead_code)]

use json_patch::merge as json_merge;

use reqwest::{Client, Method, Response};
use serde_json::{json, Value};

mod input;
mod model;
mod util;

use util::deserialize;

// TODO: Check if all fields are accessible with the public API

pub use input::get_files::{GetFiles, OrderBy, OrderDir};
pub use input::get_files_from_public_folder::GetFilesFromPublicFolder;
pub use input::update_file::UpdateFile;
pub use model::get_files::GetFilesResponse;
pub use model::get_files_from_public_folder::GetFilesFromPublicFolderResponse;
pub use model::get_files_informations::GetFilesInformationsResponse;
pub use model::get_upload_url::GetUploadUrlResponse;

use model::generic::{GenericMessageResponseWrapper, GenericUpdatedResponseWrapper};
use model::get_files::GetFilesResponseWrapper;
use model::get_files_from_public_folder::GetFilesFromPublicFolderResponseWrapper;
use model::get_files_informations::GetFilesInformationsResponseWrapper;
use model::get_upload_url::GetUploadUrlResponseWrapper;

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

        dbg!(&response);

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
                json!({ "fld_id": fld_id, "destination_fld_id": destination_fld_id, "action": "move" }),
            )
            .await?;

        deserialize::<GenericMessageResponseWrapper>(&response).map(|r| r.data)
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
                json!({ "file_codes": file_codes.join(","), "destination_fld_id": destination_fld_id, "action": "move" }),
            )
            .await?;

        deserialize::<GenericUpdatedResponseWrapper>(&response).map(|r| r.data.updated)
    }

    /// Copy one or multiple files to another location
    pub async fn copy_files(
        &self,
        file_codes: Vec<&str>,
        destination_fld_id: usize,
    ) -> UptoboxResult<usize> {
        let response = self
            .patch(
                "user/files",
                json!({ "file_codes": file_codes.join(","), "destination_fld_id": destination_fld_id, "action": "copy" }),
            )
            .await?;

        deserialize::<GenericUpdatedResponseWrapper>(&response).map(|r| r.data.updated)
    }

    /// Rename a folder
    pub async fn rename_folder(
        &self,
        fld_id: usize,
        new_name: impl Into<String>,
    ) -> UptoboxResult<String> {
        let response = self
            .patch(
                "user/files",
                json!({ "fld_id": fld_id, "new_name": new_name.into() }),
            )
            .await?;

        deserialize::<GenericMessageResponseWrapper>(&response).map(|r| r.data)
    }

    /// Create a folder
    pub async fn create_folder(
        &self,
        path: impl Into<String>,
        name: impl Into<String>,
    ) -> UptoboxResult<String> {
        let response = self
            .put(
                "user/files",
                json!({ "path": path.into(), "name": name.into() }),
            )
            .await?;

        deserialize::<GenericMessageResponseWrapper>(&response).map(|r| r.data)
    }

    /// Delete one or multiple files
    pub async fn delete_files(&self, file_codes: Vec<&str>) -> UptoboxResult<usize> {
        let response = self
            .delete("user/files", json!({ "file_codes": file_codes.join(",") }))
            .await?;

        deserialize::<GenericUpdatedResponseWrapper>(&response).map(|r| r.data.updated)
    }

    /// Delete a folder
    pub async fn delete_folder(&self, fld_id: usize) -> UptoboxResult<String> {
        let response = self
            .delete("user/files", json!({ "fld_id": fld_id }))
            .await?;

        deserialize::<GenericMessageResponseWrapper>(&response).map(|r| r.data)
    }

    /// Retrieve file informations
    ///
    /// For each file code provided, you can add a password separated by ':' For example : filecode1:password1,filecode2:password2
    pub async fn get_files_informations(
        &self,
        file_codes: Vec<&str>,
    ) -> UptoboxResult<Vec<GetFilesInformationsResponse>> {
        let response = self
            .public_get("user/public", json!({ "fileCodes": file_codes.join(",") }))
            .await?;

        deserialize::<GetFilesInformationsResponseWrapper>(&response).map(|r| r.data.list)
    }

    /// Retrieve files in public folder
    pub async fn get_files_from_public_folder(
        &self,
        get_files_from_public_folder: &GetFilesFromPublicFolder,
    ) -> UptoboxResult<Vec<GetFilesFromPublicFolderResponse>> {
        let response = self
            .public_get(
                "user/public",
                serde_json::to_value(get_files_from_public_folder).map_err(Error::ParseInput)?,
            )
            .await?;

        deserialize::<GetFilesFromPublicFolderResponseWrapper>(&response).map(|r| r.data.list)
    }

    /// Retrieve an upload url
    pub async fn get_upload_url(&self) -> UptoboxResult<GetUploadUrlResponse> {
        let response = self.get("upload", json!({})).await?;

        deserialize::<GetUploadUrlResponseWrapper>(&response).map(|r| r.data)
    }
}

impl Uptobox {
    pub fn new(key: &'static str) -> Self {
        Self {
            client: Client::new(),
            key,
        }
    }

    /// Make a Delete request
    async fn delete(&self, path: impl Into<String>, body: Value) -> UptoboxResult<String> {
        self.req(Method::DELETE, path, body).await
    }

    /// Make a Put request
    async fn put(&self, path: impl Into<String>, body: Value) -> UptoboxResult<String> {
        self.req(Method::PUT, path, body).await
    }

    /// Make a Patch request
    async fn patch(&self, path: impl Into<String>, body: Value) -> UptoboxResult<String> {
        self.req(Method::PATCH, path, body).await
    }

    /// Default implementation of request
    async fn req(
        &self,
        method: Method,
        path: impl Into<String>,
        body: Value,
    ) -> UptoboxResult<String> {
        let body = self.add_token_auth(body);

        let res = self
            .client
            .request(method, format!("{BASE_URL}{}", path.into()))
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
