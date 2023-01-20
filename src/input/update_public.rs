// use serde::Serialize;

// #[derive(Serialize, Debug)]
// pub struct UpdatePublic {
//     // The file code
//     file_codes: String,

//     // New file name value
//     #[serde(skip_serializing_if = "Option::is_none")]
//     public: usize,
// }

// impl UpdatePublic {
//     /// Create a new UpdateFile
//     ///
//     /// 1 param is requiered for the request to succeed
//     pub fn new(file_code: impl Into<String>) -> Self {
//         Self {
//             file_code: file_code.into(),
//             ..Default::default()
//         }
//     }
// }

// impl Default for UpdatePublic {
//     fn default() -> Self {
//         Self {
//             file_code: "".into(),
//             new_name: None,
//             description: None,
//             password: None,
//             public: None,
//         }
//     }
// }
