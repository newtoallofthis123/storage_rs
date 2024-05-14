use serde::Deserialize;

#[derive(Debug)]
pub enum StorageError {
    MalformerdProjectUrl(String),
    ReqwestError(reqwest::Error),
    TextParseError,
    ResponseError(ResError),
    BucketParseError(reqwest::Error),
    SerdeParseError(serde_json::Error),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ResError {
    pub statusCode: String,
    pub error: String,
    pub message: String,
}
