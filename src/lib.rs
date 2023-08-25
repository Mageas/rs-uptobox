#![allow(dead_code)]

use json_patch::merge as json_merge;

use reqwest::{Client, Method, Response};
use serde_json::{json, Value};

mod input;
mod model;
mod util;

use util::deserialize;

pub use input::get_download_url::GetDownloadUrl;
pub use input::get_files::{GetFiles, OrderBy, OrderDir};
pub use input::get_files_from_public_folder::GetFilesFromPublicFolder;
pub use input::update_file::UpdateFile;
pub use model::get_account::GetAccountResponse;
pub use model::get_account_payments::GetAccountPaymentsResponse;
pub use model::get_download_url::{GetDownloadUrlResponse, GetDownloadUrlLink, GetDownloadUrlWait};
pub use model::get_files::{GetFilesResponse, GetFilesCurrentFolder, GetFilesFolders, GetFilesFiles};
pub use model::get_files_from_public_folder::GetFilesFromPublicFolderResponse;
pub use model::get_files_informations::{GetFilesInformationsResponse, GetFilesInformationsError};
pub use model::get_upload_url::GetUploadUrlResponse;

use model::generic::GenericEmpyDataResponseWrapper;
use model::generic::{GenericMessageResponseWrapper, GenericUpdatedResponseWrapper};
use model::get_account::GetAccountResponseWrapper;
use model::get_account_payments::GetAccountPaymentsResponseWrapper;
use model::get_download_url::GetDownloadUrlResponseWrapper;
use model::get_files::GetFilesResponseWrapper;
use model::get_files_from_public_folder::GetFilesFromPublicFolderResponseWrapper;
use model::get_files_informations::GetFilesInformationsResponseWrapper;
use model::get_upload_url::GetUploadUrlResponseWrapper;

const BASE_URL: &str = "https://uptobox.com/api/";

/// Uptobox client
pub struct Uptobox {
    client: Client,
    key: &'static str,
}

/// My account
impl Uptobox {
    /// Retrieve user data
    pub async fn get_account(&self) -> UptoboxResult<GetAccountResponse> {
        let response = self.get("user/me", json!({})).await?;

        deserialize::<GetAccountResponseWrapper>(&response).map(|r| r.data)
    }

    /// Update Direct Download
    pub async fn update_account_dd(&self, ssl: bool) -> UptoboxResult {
        let response = self
            .patch("user/settings", json!({ "directDownload": ssl as usize }))
            .await?;

        deserialize::<GenericEmpyDataResponseWrapper>(&response).map(|_| ())
    }

    /// Update Direct Download
    pub async fn update_account_security_lock(&self, ssl: bool) -> UptoboxResult {
        let response = self
            .patch("user/securityLock", json!({ "securityLock": ssl as usize }))
            .await?;

        deserialize::<GenericEmpyDataResponseWrapper>(&response).map(|_| ())
    }

    /// Retrieve user payments
    pub async fn get_account_payments(&self) -> UptoboxResult<Vec<GetAccountPaymentsResponse>> {
        let response = self.get("user/payments/get", json!({})).await?;

        deserialize::<GetAccountPaymentsResponseWrapper>(&response).map(|r| r.data.list)
    }
}

/// Generate a download link
impl Uptobox {
    /// Get a waiting token
    ///
    /// If you are a premium user, it returns the link instead of the waiting token
    pub async fn get_download_url(
        &self,
        get_waiting_token: GetDownloadUrl,
    ) -> UptoboxResult<GetDownloadUrlResponse> {
        let response = self
            .get(
                "link",
                serde_json::to_value(get_waiting_token).map_err(Error::ParseInput)?,
            )
            .await?;

        deserialize::<GetDownloadUrlResponseWrapper>(&response).map(|r| r.data)
    }

    /// Get the download link with a waiting_token
    pub async fn get_download_url_waiting_token(
        &self,
        file_code: impl Into<String>,
        waiting_token: impl Into<String>,
    ) -> UptoboxResult<GetDownloadUrlResponse> {
        let response = self
            .get(
                "link",
                json!({ "file_code": file_code.into(), "waitingToken": waiting_token.into() }),
            )
            .await?;

        deserialize::<GetDownloadUrlResponseWrapper>(&response).map(|r| r.data)
    }

    /// Get a waiting token without an account
    pub async fn public_get_download_url(
        &self,
        get_waiting_token: GetDownloadUrl,
    ) -> UptoboxResult<GetDownloadUrlResponse> {
        let response = self
            .public_get(
                "link",
                serde_json::to_value(get_waiting_token).map_err(Error::ParseInput)?,
            )
            .await?;

        deserialize::<GetDownloadUrlResponseWrapper>(&response).map(|r| r.data)
    }

    /// Get the download link with a waiting_token without an account
    pub async fn public_get_download_url_waiting_token(
        &self,
        file_code: impl Into<String>,
        waiting_token: impl Into<String>,
    ) -> UptoboxResult<GetDownloadUrlResponse> {
        let response = self
            .public_get(
                "link",
                json!({ "file_code": file_code.into(), "waitingToken": waiting_token.into() }),
            )
            .await?;

        deserialize::<GetDownloadUrlResponseWrapper>(&response).map(|r| r.data)
    }
}

/// Files
impl Uptobox {
    /// Retrieve file informations
    ///
    /// For each file code provided, you can add a password separated by ':' For example : filecode1:password1,filecode2:password2
    pub async fn get_files_informations(
        &self,
        file_codes: Vec<&str>,
    ) -> UptoboxResult<Vec<GetFilesInformationsResponse>> {
        let response = self
            .public_get("link/info", json!({ "fileCodes": file_codes.join(",") }))
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
}

/// File Management
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
}

/// Upload
impl Uptobox {
    /// Retrieve an upload url
    pub async fn get_upload_url(&self) -> UptoboxResult<GetUploadUrlResponse> {
        let response = self.get("upload", json!({})).await?;

        deserialize::<GetUploadUrlResponseWrapper>(&response).map(|r| r.data)
    }
}

/// Internal
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
    #[error("Unable to parse the input")]
    ParseInput(#[source] serde_json::Error),

    #[error("Unable to parse the response: status_code: {0}, message: {1}, data: {2}")]
    ParseResponse(usize, String, String),

    #[error("Unable to parse the response, unknown error")]
    UnknownParseResponse(#[source] serde_json::Error),

    #[error("Bad response")]
    HttpRequest(#[source] reqwest::Error),

    #[error("Bad response with status code: {0}")]
    HttpResponseCode(u16),
}

pub type UptoboxResult<T = ()> = Result<T, Error>;
