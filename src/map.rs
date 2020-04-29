use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::{layer::Layer, tileset::Tileset};

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaggerAxis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaggerIndex {
    Odd,
    Even,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "orientation")]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered {
        #[serde(rename = "staggeraxis")]
        stagger_axis: StaggerAxis,
        #[serde(rename = "staggerindex")]
        stagger_index: StaggerIndex,
    }, // Isometric (Staggered)
    Hexagonal {
        #[serde(deserialize_with = "deserialize_number_from_string")]
        #[serde(rename = "hexsidelength")]
        hexside_length: i32,
        #[serde(rename = "staggeraxis")]
        stagger_axis: StaggerAxis,
        #[serde(rename = "staggerindex")]
        stagger_index: StaggerIndex,
    }, // Hexagonal (Staggered)
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

#[derive(Debug, Deserialize)]
pub struct Map {
    pub version: String,
    #[serde(rename = "tiledversion")]
    pub tiled_version: String,
    #[serde(flatten)]
    pub orientation: Orientation,
    #[serde(rename = "renderorder")]
    pub render_order: RenderOrder,
    #[serde(rename = "compressionlevel")]
    pub compression_level: Option<usize>,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "tilewidth")]
    pub tile_width: i32,
    #[serde(rename = "tileheight")]
    pub tile_height: i32,
    pub infinite: bool,
    #[serde(rename = "backgroundcolor")]
    pub background_color: Option<String>,
    #[serde(rename = "nextlayerid")]
    pub next_layer_id: u32,
    #[serde(rename = "nextobjectid")]
    pub next_object_id: u32,
    #[serde(rename = "tileset")]
    pub tilesets: Vec<Tileset>,
    #[serde(rename = "layer")]
    pub layers: Vec<Layer>,
}
