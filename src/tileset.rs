#![allow(unknown_lints)]
#![warn(clippy::all)]

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Deserialize)]
pub struct Image {
    pub source: String,
    #[serde(rename = "trans")]
    pub transparent_color: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub width: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct Tile {
    pub id: u32,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Tileset {
    Embedded {
        #[serde(
            deserialize_with = "deserialize_number_from_string",
            rename = "firstgid"
        )]
        first_gid: u32,
        name: String,
        #[serde(
            deserialize_with = "deserialize_number_from_string",
            rename = "tilewidth"
        )]
        tile_width: u32,
        #[serde(
            deserialize_with = "deserialize_number_from_string",
            rename = "tileheight"
        )]
        tile_height: u32,
        #[serde(default, deserialize_with = "deserialize_number_from_string")]
        spacing: u32,
        #[serde(default, deserialize_with = "deserialize_number_from_string")]
        margin: u32,
        #[serde(
            deserialize_with = "deserialize_number_from_string",
            rename = "tilecount"
        )]
        tile_count: usize,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        columns: usize,
        #[serde(rename = "backgroundcolor")]
        background_color: Option<String>,
        // tileoffset
        // grid
        image: Image,
        // terrainttypes
        #[serde(rename = "tile", default)]
        tiles: Vec<Tile>,
        // wangsets
    },
    External {
        #[serde(
            deserialize_with = "deserialize_number_from_string",
            rename = "firstgid"
        )]
        first_gid: u32,
        source: String,
    },
}
