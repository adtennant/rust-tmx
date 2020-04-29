#![allow(unknown_lints)]
#![warn(clippy::all)]

pub mod layer;
pub mod map;
pub mod tileset;

use std::io::Read;
use thiserror::Error;

pub use map::Map;

#[derive(Debug, Error)]
pub enum TMXError {
    #[error("error deserializing")]
    XMLError(#[from] serde_xml_rs::Error),
}

pub fn from_reader<R: Read>(reader: R) -> Result<map::Map, TMXError> {
    let map = serde_xml_rs::from_reader(reader)?;

    Ok(map)
}

pub fn from_str(s: &str) -> Result<map::Map, TMXError> {
    let map = serde_xml_rs::from_str(s)?;

    Ok(map)
}
