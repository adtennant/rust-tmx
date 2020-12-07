use crate::{error::Error, layer, metadata, tileset};

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_aux::field_attributes::deserialize_number_from_string;

/// For staggered and hexagonal maps, determines which axis (“x” or “y”) is staggered.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StaggerAxis {
    X,
    Y,
}

/// For staggered and hexagonal maps, determines whether the “even” or “odd” indexes along the staggered axis are shifted.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StaggerIndex {
    Odd,
    Even,
}

/// Map orientation. Tiled supports “orthogonal”, “isometric”, “staggered” and “hexagonal”.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(untagged)]
pub enum TilesetKind {
    Embedded(tileset::Tileset),
    External {
        /// If this tileset is stored in an external TSX (Tile Set XML) file, this attribute refers to that file. That TSX file has the same structure as the <tileset> element described here. (There is the firstgid attribute missing and this source attribute is also not there. These two attributes are kept in the TMX map, since they are map specific.)
        source: String,
    },
}

fn default_compression_level() -> i32 {
    -1
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Map {
    #[serde(flatten)]
    pub metadata: metadata::Metadata,
    /// Map orientation. Tiled supports “orthogonal”, “isometric”, “staggered” and “hexagonal”
    #[serde(flatten)]
    pub orientation: Orientation,
    /// The order in which tiles on tile layers are rendered. Valid values are right-down (the default), right-up, left-down and left-up. In all cases, the map is drawn row-by-row. (only supported for orthogonal maps at the moment)
    #[serde(rename = "renderorder", alias = "$renderorder")]
    pub render_order: RenderOrder,
    /// The compression level to use for tile layer data (defaults to -1, which means to use the algorithm default).
    #[serde(rename = "compressionlevel", default = "default_compression_level")]
    pub compression_level: i32,
    /// The map width in tiles.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub width: i32,
    /// The map height in tiles.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub height: i32,
    /// The width of a tile.
    #[serde(
        rename = "tilewidth",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub tile_width: i32,
    /// The height of a tile.
    #[serde(
        rename = "tileheight",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub tile_height: i32,
    #[serde(default, deserialize_with = "deserialize_bool_from_anything")]
    pub infinite: bool,
    /// The background color of the map. (optional, may include alpha value since 0.15 in the form #AARRGGBB)
    #[serde(rename = "backgroundcolor", alias = "$backgroundcolor")]
    pub background_color: Option<String>,
    /// Stores the next available ID for new layers. This number is stored to prevent reuse of the same ID after layers have been removed. (since 1.2)
    #[serde(
        rename = "nextlayerid",
        deserialize_with = "deserialize_number_from_string",
        default
    )]
    pub next_layer_id: u32,
    /// Stores the next available ID for new objects. This number is stored to prevent reuse of the same ID after objects have been removed. (since 0.11)
    #[serde(
        rename = "nextobjectid",
        deserialize_with = "deserialize_number_from_string",
        default
    )]
    pub next_object_id: u32,
    #[serde(alias = "layer")]
    pub layers: Vec<layer::Layer>,
    #[serde(alias = "tileset")]
    pub tilesets: Vec<Tileset>,
}

impl Map {
    pub fn from_json(s: &str) -> Result<Map, Error> {
        serde_json::from_str(s).map_err(From::from)
    }

    pub fn from_json_data(buf: &[u8]) -> Result<Map, Error> {
        let s = std::str::from_utf8(buf).map_err(Error::Utf8Error)?;
        Map::from_json(s)
    }

    #[cfg(feature = "xml")]
    pub fn from_xml(s: &str) -> Result<Map, Error> {
        #[derive(Deserialize)]
        struct Doc {
            map: Vec<Map>,
        }

        let json = super::to_json::to_json(s).map_err(Error::Conversion)?;
        let mut doc: Doc = serde_json::from_value(json).map_err(Error::Deserialization)?;

        Ok(doc.map.remove(0))
    }

    #[cfg(feature = "xml")]
    pub fn from_xml_data(buf: &[u8]) -> Result<Map, Error> {
        let s = std::str::from_utf8(buf).map_err(Error::Utf8Error)?;
        Map::from_xml(s)
    }
}
