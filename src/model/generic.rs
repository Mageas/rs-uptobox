use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponseWrapper {
    pub data: GenericResponse,
}

#[derive(Deserialize, Debug)]
pub struct GenericResponse {
    #[serde(deserialize_with = "bool_from_int")]
    pub updated: bool,
}

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
