use crate::to_json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error deserializing")]
    Deserialization(#[from] serde_json::Error),
    #[error("error converting to JSON")]
    Conversion(#[from] to_json::Error),
    #[error("error converting to UTF8")]
    Utf8Error(#[from] std::str::Utf8Error),
}
