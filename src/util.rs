use crate::{Error, UptoboxResult};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDeserialize {
    pub status_code: usize,
    pub message: String,
    pub data: Option<String>,
}

pub fn deserialize<'de, T>(json: &'de str) -> UptoboxResult<T>
where
    T: serde::Deserialize<'de> + DeserializeCheck,
{
    match serde_json::from_str::<T>(json) {
        Ok(r) => match r.status_code() {
            0 => Ok(r),
            _ => Err(Error::ParseResponse(
                r.status_code(),
                String::new(), // r.data(),
                r.message().unwrap_or_default(),
            )),
        },
        Err(e) => match serde_json::from_str::<ErrorDeserialize>(json) {
            Ok(r) => Err(Error::ParseResponse(
                r.status_code,
                r.message,
                r.data.unwrap_or_default(),
            )),
            Err(_) => Err(Error::UnknownParseResponse(e)),
        },
    }
}

pub trait DeserializeCheck {
    fn status_code(&self) -> usize;
    fn data(&self) -> String;
    fn message(&self) -> Option<String>;
}
