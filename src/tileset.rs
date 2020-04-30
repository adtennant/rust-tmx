#![allow(unknown_lints)]
#![warn(clippy::all)]

use crate::{error::TMXError, metadata::Metadata};

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Image {
    /// The reference to the tileset image file (Tiled supports most common image formats).
    pub source: String,
    /// Defines a specific color that is treated as transparent (example value: “#FF00FF” for magenta). Up until Tiled 0.12, this value is written out without a # but this is planned to change.
    #[serde(rename = "trans")]
    pub transparent_color: Option<String>,
    /// The image width in pixels (optional, used for tile index correction when the image changes)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub width: u32,
    /// The image height in pixels (optional)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct Tile {
    /// The local tile ID within its tileset.
    pub id: u32,
    /// The type of the tile. Refers to an object type and is used by tile objects. (optional) (since 1.0)
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Tileset {
    #[serde(flatten)]
    pub metadata: Option<Metadata>,
    /// The name of this tileset.
    pub name: String,
    #[serde(
        deserialize_with = "deserialize_number_from_string",
        rename = "tilewidth"
    )]
    /// The (maximum) width of the tiles in this tileset.
    pub tile_width: u32,
    /// The (maximum) height of the tiles in this tileset.
    #[serde(
        deserialize_with = "deserialize_number_from_string",
        rename = "tileheight"
    )]
    pub tile_height: u32,
    /// The spacing in pixels between the tiles in this tileset (applies to the tileset image).
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub spacing: u32,
    /// The margin around the tiles in this tileset (applies to the tileset image).
    #[serde(default, deserialize_with = "deserialize_number_from_string")]
    pub margin: u32,
    /// The number of tiles in this tileset (since 0.13)
    #[serde(
        deserialize_with = "deserialize_number_from_string",
        rename = "tilecount"
    )]
    pub tile_count: usize,
    /// The number of tile columns in the tileset. For image collection tilesets it is editable and is used when displaying the tileset. (since 0.15)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub columns: u32,
    #[serde(rename = "backgroundcolor")]
    pub background_color: Option<String>,
    // tileoffset
    // grid
    pub image: Image,
    // terrainttypes
    #[serde(rename = "tile", default)]
    pub tiles: Vec<Tile>,
    // wangsets
}

impl Tileset {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, TMXError> {
        let tileset = serde_xml_rs::from_reader(reader)?;

        Ok(tileset)
    }

    pub fn from_str(s: &str) -> Result<Self, TMXError> {
        let tileset = serde_xml_rs::from_str(s)?;

        Ok(tileset)
    }
}
