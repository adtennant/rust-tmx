use crate::{error::TMXError, layer::Layer, metadata::Metadata, tileset};

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::io::Read;

/// For staggered and hexagonal maps, determines which axis (“x” or “y”) is staggered.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaggerAxis {
    X,
    Y,
}

/// For staggered and hexagonal maps, determines whether the “even” or “odd” indexes along the staggered axis are shifted.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaggerIndex {
    Odd,
    Even,
}

/// Map orientation. Tiled supports “orthogonal”, “isometric”, “staggered” and “hexagonal”.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "orientation")]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered {
        /// For staggered and hexagonal maps, determines which axis (“x” or “y”) is staggered.
        #[serde(rename = "staggeraxis")]
        stagger_axis: StaggerAxis,
        /// For staggered and hexagonal maps, determines whether the “even” or “odd” indexes along the staggered axis are shifted.
        #[serde(rename = "staggerindex")]
        stagger_index: StaggerIndex,
    }, // Isometric (Staggered)
    Hexagonal {
        /// Only for hexagonal maps. Determines the width or height (depending on the staggered axis) of the tile’s edge, in pixels.
        #[serde(deserialize_with = "deserialize_number_from_string")]
        #[serde(rename = "hexsidelength")]
        hexside_length: i32,
        /// For staggered and hexagonal maps, determines which axis (“x” or “y”) is staggered.
        #[serde(rename = "staggeraxis")]
        stagger_axis: StaggerAxis,
        /// For staggered and hexagonal maps, determines whether the “even” or “odd” indexes along the staggered axis are shifted.
        #[serde(rename = "staggerindex")]
        stagger_index: StaggerIndex,
    }, // Hexagonal (Staggered)
}

/// The order in which tiles on tile layers are rendered. Valid values are right-down (the default), right-up, left-down and left-up. In all cases, the map is drawn row-by-row. (only supported for orthogonal maps at the moment)
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

#[derive(Debug, Deserialize)]
pub struct Tileset {
    /// The first global tile ID of this tileset (this global ID maps to the first tile in this tileset).
    #[serde(
        deserialize_with = "deserialize_number_from_string",
        rename = "firstgid"
    )]
    pub first_gid: u32,
    #[serde(flatten)]
    pub kind: TilesetKind,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TilesetKind {
    Embedded(tileset::Tileset),
    External {
        /// If this tileset is stored in an external TSX (Tile Set XML) file, this attribute refers to that file. That TSX file has the same structure as the <tileset> element described here. (There is the firstgid attribute missing and this source attribute is also not there. These two attributes are kept in the TMX map, since they are map specific.)
        source: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct Map {
    #[serde(flatten)]
    pub metadata: Metadata,
    /// Map orientation. Tiled supports “orthogonal”, “isometric”, “staggered” and “hexagonal”
    #[serde(flatten)]
    pub orientation: Orientation,
    /// The order in which tiles on tile layers are rendered. Valid values are right-down (the default), right-up, left-down and left-up. In all cases, the map is drawn row-by-row. (only supported for orthogonal maps at the moment)
    #[serde(rename = "renderorder")]
    pub render_order: RenderOrder,
    /// The compression level to use for tile layer data (defaults to -1, which means to use the algorithm default).
    #[serde(rename = "compressionlevel")]
    pub compression_level: Option<usize>,
    /// The map width in tiles.
    pub width: i32,
    /// The map height in tiles.
    pub height: i32,
    /// The width of a tile.
    #[serde(rename = "tilewidth")]
    pub tile_width: i32,
    /// The height of a tile.
    #[serde(rename = "tileheight")]
    pub tile_height: i32,
    pub infinite: bool,
    /// The background color of the map. (optional, may include alpha value since 0.15 in the form #AARRGGBB)
    #[serde(rename = "backgroundcolor")]
    pub background_color: Option<String>,
    /// Stores the next available ID for new layers. This number is stored to prevent reuse of the same ID after layers have been removed. (since 1.2)
    #[serde(rename = "nextlayerid")]
    pub next_layer_id: u32,
    /// Stores the next available ID for new objects. This number is stored to prevent reuse of the same ID after objects have been removed. (since 0.11)
    #[serde(rename = "nextobjectid")]
    pub next_object_id: u32,
    #[serde(rename = "tileset")]
    pub tilesets: Vec<Tileset>,
    #[serde(rename = "layer")]
    pub layers: Vec<Layer>,
}

impl Map {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, TMXError> {
        let map = serde_xml_rs::from_reader(reader)?;

        Ok(map)
    }

    pub fn from_str(s: &str) -> Result<Self, TMXError> {
        let map = serde_xml_rs::from_str(s)?;

        Ok(map)
    }
}
