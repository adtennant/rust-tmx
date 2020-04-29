use thiserror::Error;

#[derive(Debug, Error)]
pub enum TMXError {
    #[error("error deserializing")]
    XMLError(#[from] serde_xml_rs::Error),
}
