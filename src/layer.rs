use serde::{de::Deserializer, Deserialize};
use serde_aux::field_attributes::deserialize_number_from_string;

fn parse_csv(value: String) -> Result<Vec<Tile>, String> {
    Ok(value
        .split('\n')
        .filter(|s| s.trim() != "")
        .flat_map(|s| s.split(','))
        .filter(|s| s.trim() != "")
        .map(|gid| gid.trim().parse())
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|gid| Tile { gid })
        .collect())
}

#[cfg(feature = "base64-data")]
fn decode_base64(value: String) -> Result<Vec<u8>, String> {
    let data = base64::decode(value.trim().as_bytes()).map_err(|e| e.to_string())?;

    Ok(data)
}

#[cfg(feature = "gzip-data")]
fn decode_gzip(data: Vec<u8>) -> Result<Vec<u8>, String> {
    use libflate::gzip::Decoder;
    use std::io::Read;

    let mut decoder = Decoder::new(data.as_slice()).map_err(|e| e.to_string())?;
    let mut data = Vec::new();
    decoder.read_to_end(&mut data).map_err(|e| e.to_string())?;

    Ok(data)
}

#[cfg(feature = "zlib-data")]
fn decode_zlib(data: Vec<u8>) -> Result<Vec<u8>, String> {
    use libflate::zlib::Decoder;
    use std::io::Read;

    let mut decoder = Decoder::new(data.as_slice()).map_err(|e| e.to_string())?;
    let mut data = Vec::new();
    decoder.read_to_end(&mut data).map_err(|e| e.to_string())?;

    Ok(data)
}

#[cfg(feature = "zstd-data")]
fn decode_zstd(data: Vec<u8>) -> Result<Vec<u8>, String> {
    let data = zstd::stream::decode_all(data.as_slice()).map_err(|e| e.to_string())?;

    Ok(data)
}

#[cfg(feature = "base64-data")]
fn parse_base64_data(data: Vec<u8>) -> Result<Vec<Tile>, String> {
    use std::convert::TryInto;

    Ok(data
        .chunks(4)
        .map(|chunk| chunk.try_into())
        .collect::<Result<Vec<[u8; 4]>, _>>()
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(u32::from_le_bytes)
        .map(|gid| Tile { gid })
        .collect::<Vec<_>>())
}

fn decode_tile_data(
    encoding: Option<String>,
    #[allow(unused_variables)] compression: Option<String>,
    data: String,
) -> Result<Vec<Tile>, String> {
    match encoding.as_deref() {
        Some("csv") => parse_csv(data),
        #[cfg(feature = "base64-data")]
        Some("base64") => decode_base64(data)
            .and_then(|data| match compression.as_deref() {
                None => Ok(data),
                #[cfg(feature = "gzip-data")]
                Some("gzip") => decode_gzip(data),
                #[cfg(feature = "zlib-data")]
                Some("zlib") => decode_zlib(data),
                #[cfg(feature = "zstd-data")]
                Some("zstd") => decode_zstd(data),
                Some(compression) => Err(format!("invalid compression: {}", compression)),
            })
            .and_then(parse_base64_data),
        None => Err("missing encoding".into()),
        Some(encoding) => Err(format!("invalid encoding: {:?}", encoding)),
    }
}

const FLIPPED_HORIZONTALLY_FLAG: u32 = 0x8000_0000;
const FLIPPED_VERTICALLY_FLAG: u32 = 0x4000_0000;
const FLIPPED_DIAGONALLY_FLAG: u32 = 0x2000_0000;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Tile {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    gid: u32,
}

impl Tile {
    /// The global tile ID (default: 0).
    pub fn gid(self) -> u32 {
        self.gid & !(FLIPPED_HORIZONTALLY_FLAG | FLIPPED_VERTICALLY_FLAG | FLIPPED_DIAGONALLY_FLAG)
    }

    /// Whether the tile is horizontally flipped.
    pub fn flipped_horizontally(self) -> bool {
        self.gid & FLIPPED_HORIZONTALLY_FLAG > 0
    }

    /// Whether the tile is vertically flipped.
    pub fn flipped_vertically(self) -> bool {
        self.gid & FLIPPED_VERTICALLY_FLAG > 0
    }

    /// Whether the tile is flipped (anti) diagonally, enabling tile rotation.
    pub fn flipped_diagonally(self) -> bool {
        self.gid & FLIPPED_DIAGONALLY_FLAG > 0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Chunk {
    /// The x coordinate of the chunk in tiles.
    pub x: u32,
    /// The y coordinate of the chunk in tiles.
    pub y: u32,
    /// The width of the chunk in tiles.
    pub width: u32,
    /// The height of the chunk in tiles.
    pub height: u32,
    pub data: Vec<Tile>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LayerData {
    Tiles(Vec<Tile>),
    Chunks(Vec<Chunk>),
}

impl<'de> Deserialize<'de> for LayerData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::convert::TryInto;

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum XMLTileData {
            Data {
                encoding: Option<String>,
                compression: Option<String>,
                #[serde(rename = "_")]
                data: String,
            },
            Tiles {
                #[serde(rename = "tile")]
                tiles: Vec<Tile>,
            },
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum JSONTileData {
            Vec(Vec<u32>),
            String(String),
        }

        #[derive(Deserialize)]
        struct XMLChunk {
            #[serde(deserialize_with = "deserialize_number_from_string")]
            x: u32,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            y: u32,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            width: u32,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            height: u32,
            #[serde(flatten)]
            data: XMLTileData,
        }

        #[derive(Deserialize)]
        struct XMLChunks {
            encoding: Option<String>,
            compression: Option<String>,
            #[serde(rename = "chunk")]
            chunks: Vec<XMLChunk>,
        }

        #[derive(Deserialize)]
        struct JSONChunk {
            #[serde(deserialize_with = "deserialize_number_from_string")]
            x: u32,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            y: u32,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            width: u32,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            height: u32,
            data: JSONTileData,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Data {
            XML {
                data: Vec<XMLTileData>,
            },
            XMLChunks {
                data: Vec<XMLChunks>,
            },
            JSON {
                encoding: Option<String>,
                compression: Option<String>,
                data: JSONTileData,
            },
            JSONChunks {
                encoding: Option<String>,
                compression: Option<String>,
                chunks: Vec<JSONChunk>,
            },
        }

        impl TryInto<LayerData> for Data {
            type Error = String;

            fn try_into(self) -> Result<LayerData, Self::Error> {
                match self {
                    Data::XML { mut data } => match data.remove(0) {
                        XMLTileData::Data {
                            encoding,
                            compression,
                            data,
                        } => decode_tile_data(encoding, compression, data).map(LayerData::Tiles),
                        XMLTileData::Tiles { tiles } => Ok(LayerData::Tiles(tiles)),
                    },
                    Data::XMLChunks { mut data, .. } => {
                        let XMLChunks {
                            encoding,
                            compression,
                            chunks,
                        } = data.remove(0);
                        let chunks = chunks
                            .into_iter()
                            .map(|chunk| {
                                let data = match chunk.data {
                                    XMLTileData::Data { data, .. } => decode_tile_data(
                                        encoding.clone(),
                                        compression.clone(),
                                        data,
                                    ),
                                    XMLTileData::Tiles { tiles } => Ok(tiles),
                                }?;

                                Ok(Chunk {
                                    x: chunk.x,
                                    y: chunk.y,
                                    width: chunk.width,
                                    height: chunk.height,
                                    data,
                                })
                            })
                            .collect::<Result<Vec<_>, String>>()?;

                        Ok(LayerData::Chunks(chunks))
                    }
                    Data::JSON {
                        data: JSONTileData::Vec(gids),
                        ..
                    } => Ok(LayerData::Tiles(
                        gids.into_iter().map(|gid| Tile { gid }).collect(),
                    )),
                    Data::JSON {
                        encoding,
                        compression,
                        data: JSONTileData::String(data),
                    } => decode_tile_data(encoding, compression, data).map(LayerData::Tiles),
                    Data::JSONChunks {
                        encoding,
                        compression,
                        chunks,
                    } => {
                        let chunks = chunks
                            .into_iter()
                            .map(|chunk| {
                                let data = match chunk.data {
                                    JSONTileData::Vec(gids) => {
                                        Ok(gids.into_iter().map(|gid| Tile { gid }).collect())
                                    }
                                    JSONTileData::String(data) => decode_tile_data(
                                        encoding.clone(),
                                        compression.clone(),
                                        data,
                                    ),
                                }?;

                                Ok(Chunk {
                                    x: chunk.x,
                                    y: chunk.y,
                                    width: chunk.width,
                                    height: chunk.height,
                                    data,
                                })
                            })
                            .collect::<Result<Vec<_>, String>>()?;

                        Ok(LayerData::Chunks(chunks))
                    }
                }
            }
        }

        let data = Data::deserialize(deserializer)?;
        data.try_into().map_err(serde::de::Error::custom)
    }
}

fn default_visible() -> bool {
    true
}

fn default_opacity() -> f64 {
    1.0
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Layer {
    /// Unique ID of the layer. Each layer that added to a map gets a unique id. Even if a layer is deleted, no layer ever gets the same ID. Can not be changed in Tiled. (since Tiled 1.2)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u32,
    /// The name of the layer.
    #[serde(default)]
    pub name: String,
    /// The x coordinate of the layer in tiles. Defaults to 0 and can not be changed in Tiled.
    #[serde(default)]
    pub x: i32,
    /// The y coordinate of the layer in tiles. Defaults to 0 and can not be changed in Tiled.
    #[serde(default)]
    pub y: i32,
    /// The width of the layer in tiles. Always the same as the map width for fixed-size maps.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub width: i32,
    /// The height of the layer in tiles. Always the same as the map height for fixed-size maps.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub height: i32,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default)]
    pub locked: bool,
    /// The opacity of the layer as a value from 0 to 1. Defaults to 1.
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    /// Rendering offset for this layer in pixels. Defaults to 0. (since 0.14)
    #[serde(default, rename = "offsetx")]
    pub offset_x: f64,
    ///  Rendering offset for this layer in pixels. Defaults to 0. (since 0.14)
    #[serde(default, rename = "offsety")]
    pub offset_y: f64,
    #[serde(flatten)]
    pub data: LayerData,
}
