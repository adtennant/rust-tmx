use crate::{error::Error, metadata};

use serde::{de::Deserializer, Deserialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Image {
    /// The reference to the tileset image file (Tiled supports most common image formats).
    pub source: String,
    /// Defines a specific color that is treated as transparent (example value: “#FF00FF” for magenta). Up until Tiled 0.12, this value is written out without a # but this is planned to change.
    pub transparent_color: Option<String>,
    /// The image width in pixels (optional, used for tile index correction when the image changes)
    pub width: u32,
    /// The image height in pixels (optional)
    pub height: u32,
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct XMLImage {
            pub source: String,
            pub trans: Option<String>,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            pub width: u32,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            pub height: u32,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ImageData {
            XML {
                image: Vec<XMLImage>,
            },
            JSON {
                image: String,
                imageheight: u32,
                imagewidth: u32,
                transparentcolor: Option<String>,
            },
        }

        impl Into<Image> for ImageData {
            fn into(self) -> Image {
                match self {
                    ImageData::XML { mut image } => {
                        let image = image.remove(0);

                        Image {
                            source: image.source,
                            transparent_color: image.trans,
                            width: image.width,
                            height: image.height,
                        }
                    }
                    ImageData::JSON {
                        image,
                        imageheight,
                        imagewidth,
                        transparentcolor,
                    } => Image {
                        source: image,
                        transparent_color: transparentcolor,
                        width: imagewidth,
                        height: imageheight,
                    },
                }
            }
        }

        let data = ImageData::deserialize(deserializer)?;
        Ok(data.into())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Tile {
    /// The local tile ID within its tileset.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u32,
    /// The type of the tile. Refers to an object type and is used by tile objects. (optional) (since 1.0)
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Tileset {
    #[serde(flatten)]
    pub metadata: Option<metadata::Metadata>,
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
    #[serde(flatten)]
    pub image: Image,
    // terrainttypes
    #[serde(alias = "tile", default)]
    pub tiles: Vec<Tile>,
    // wangsets
}

impl Tileset {
    pub fn from_json(s: &str) -> Result<Tileset, Error> {
        serde_json::from_str(s).map_err(From::from)
    }

    #[cfg(feature = "xml")]
    pub fn from_xml(s: &str) -> Result<Tileset, Error> {
        #[derive(Deserialize)]
        struct Doc {
            tileset: Vec<Tileset>,
        }

        let json = super::to_json::to_json(s).map_err(Error::Conversion)?;
        let mut doc: Doc = serde_json::from_value(json).map_err(Error::Deserialization)?;

        Ok(doc.tileset.remove(0))
    }
}
