#![allow(unknown_lints)]
#![warn(clippy::all)]

#[cfg(feature = "xml")]
mod to_json;

pub mod error;
pub mod layer;
pub mod map;
pub mod metadata;
pub mod tileset;

pub use map::Map;
pub use tileset::Tileset;
