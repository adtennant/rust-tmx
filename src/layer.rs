use serde::{de, Deserialize, Deserializer};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::{error, str::FromStr};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    #[cfg(feature = "base64-data")]
    Base64(Option<Compression>),
    CSV,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    GZip,
    ZLib,
    ZStd,
}

impl FromStr for Compression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gzip" => Ok(Compression::GZip),
            "zlib" => Ok(Compression::ZLib),
            "zstd" => Ok(Compression::ZStd),
            _ => Err(format!("invalid compression: {}", s)),
        }
    }
}

const FLIPPED_HORIZONTALLY_FLAG: u32 = 0x80000000;
const FLIPPED_VERTICALLY_FLAG: u32 = 0x40000000;
const FLIPPED_DIAGONALLY_FLAG: u32 = 0x20000000;

#[derive(Debug, Deserialize)]
pub struct Tile {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    gid: u32,
}

impl Tile {
    pub fn gid(&self) -> u32 {
        self.gid & !(FLIPPED_HORIZONTALLY_FLAG | FLIPPED_VERTICALLY_FLAG | FLIPPED_DIAGONALLY_FLAG)
    }

    pub fn flipped_horizontally(&self) -> bool {
        self.gid & FLIPPED_HORIZONTALLY_FLAG > 0
    }

    pub fn flipped_vertically(&self) -> bool {
        self.gid & FLIPPED_VERTICALLY_FLAG > 0
    }

    pub fn flipped_diagonally(&self) -> bool {
        self.gid & FLIPPED_DIAGONALLY_FLAG > 0
    }
}

#[derive(Debug, Deserialize)]
pub struct Chunk {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub x: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub y: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub width: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub height: usize,
    #[serde(rename = "tile")]
    pub tiles: Vec<Tile>,
}

#[derive(Debug)]
pub enum Values {
    Tiles(Vec<Tile>),
    Chunks(Vec<Chunk>),
}

#[derive(Debug)]
pub struct Data {
    pub encoding: Option<Encoding>,
    pub values: Values,
}

fn parse_csv(value: &String) -> Result<Vec<Tile>, Box<dyn error::Error>> {
    Ok(value
        .split("\n")
        .filter(|s| s.trim() != "")
        .flat_map(|s| s.split(","))
        .filter(|s| s.trim() != "")
        .map(|gid| gid.trim().parse())
        .collect::<Result<Vec<u32>, _>>()?
        .into_iter()
        .map(|gid| Tile { gid })
        .collect())
}

#[cfg(feature = "base64-data")]
fn decode_base64(value: &String) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let data = base64::decode(value.trim().as_bytes())?;

    Ok(data)
}

#[cfg(feature = "gzip-data")]
fn decode_gzip(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn error::Error>> {
    use libflate::gzip::Decoder;
    use std::io::Read;

    let mut decoder = Decoder::new(data.as_slice())?;
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    Ok(data)
}

#[cfg(feature = "zlib-data")]
fn decode_zlib(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn error::Error>> {
    use libflate::zlib::Decoder;
    use std::io::Read;

    let mut decoder = Decoder::new(data.as_slice())?;
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    Ok(data)
}

#[cfg(feature = "zstd-data")]
fn decode_zstd(data: Vec<u8>) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let data = zstd::stream::decode_all(data.as_slice())?;

    Ok(data)
}

#[cfg(feature = "base64-data")]
fn parse_base64_data(data: Vec<u8>) -> Result<Vec<Tile>, Box<dyn error::Error>> {
    use std::convert::TryInto;

    Ok(data
        .chunks(4)
        .map(|chunk| chunk.try_into())
        .collect::<Result<Vec<[u8; 4]>, _>>()?
        .into_iter()
        .map(|chunk| u32::from_le_bytes(chunk))
        .map(|gid| Tile { gid })
        .collect::<Vec<_>>())
}

#[derive(Debug, Error)]
enum DisabledFeatureError {
    #[error("the base64-data feature is not enabled")]
    Base64,
    #[error("the gzip-data feature is not enabled")]
    GZip,
    #[error("the zlib-data feature is not enabled")]
    ZLib,
    #[error("the zstd-data feature is not enabled")]
    ZStd,
}

fn decode_tile_data(data: &String, encoding: Encoding) -> Result<Vec<Tile>, Box<dyn error::Error>> {
    match encoding {
        Encoding::CSV => parse_csv(data),

        #[cfg(feature = "base64-data")]
        Encoding::Base64(compression) => decode_base64(data)
            .and_then(|data| match compression {
                None => Ok(data),
                Some(compression) => match compression {
                    #[cfg(feature = "gzip-data")]
                    Compression::GZip => decode_gzip(data),

                    #[cfg(feature = "zlib-data")]
                    Compression::ZLib => decode_zlib(data),

                    #[cfg(feature = "zstd-data")]
                    Compression::ZStd => decode_zstd(data),

                    #[allow(unreachable_patterns)]
                    _ => Err(match compression {
                        Compression::GZip => DisabledFeatureError::GZip,
                        Compression::ZLib => DisabledFeatureError::ZLib,
                        Compression::ZStd => DisabledFeatureError::ZStd,
                    }
                    .into()),
                },
            })
            .and_then(parse_base64_data),

        #[allow(unreachable_patterns)]
        _ => Err(DisabledFeatureError::Base64.into()),
    }
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawChunk {
            #[serde(deserialize_with = "deserialize_number_from_string")]
            x: usize,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            y: usize,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            width: usize,
            #[serde(deserialize_with = "deserialize_number_from_string")]
            height: usize,
            #[serde(rename = "tile")]
            tiles: Option<Vec<Tile>>,
            #[serde(flatten)]
            other: std::collections::HashMap<String, String>,
        }

        #[derive(Debug, Deserialize)]
        struct RawData {
            #[serde(rename = "chunk")]
            chunks: Option<Vec<RawChunk>>,
            #[serde(rename = "tile")]
            tiles: Option<Vec<Tile>>,
            #[serde(flatten)]
            other: std::collections::HashMap<String, String>,
        }

        let RawData {
            chunks,
            tiles,
            other,
        } = RawData::deserialize(deserializer)?;

        let encoding = other
            .get("encoding")
            .map_or(Ok(None), |encoding| match encoding.as_str() {
                "csv" => Ok(Some(Encoding::CSV)),
                "base64" => match other.get("compression") {
                    None => Ok(Some(Encoding::Base64(None))),
                    Some(compression) => Compression::from_str(compression.as_str())
                        .map(|compression| Some(Encoding::Base64(Some(compression)))),
                },
                _ => Err(format!("invalid encoding: {}", encoding)),
            })
            .map_err(de::Error::custom)?;

        let values = match (encoding, chunks, tiles, other.get("$value")) {
            // XML tiles
            (None, None, Some(tiles), None) => Ok(Values::Tiles(tiles)),

            // XML chunks
            (None, Some(chunks), None, None) => chunks
                .into_iter()
                .map(|chunk| match chunk.tiles {
                    Some(tiles) => Ok(Chunk {
                        x: chunk.x,
                        y: chunk.y,
                        width: chunk.width,
                        height: chunk.height,
                        tiles: tiles,
                    }),
                    None => Err(String::from("invalid chunk data")),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|chunks| Values::Chunks(chunks)),

            // Encoded tiles
            (Some(encoding), None, None, Some(value)) => decode_tile_data(value, encoding)
                .map(Values::Tiles)
                .map_err(|e| e.to_string()),

            // Encoded chunks
            (Some(encoding), Some(chunks), None, None) => chunks
                .into_iter()
                .map(|chunk| {
                    chunk
                        .other
                        .get("$value")
                        .ok_or(String::from("missing chunk data"))
                        .and_then(|data| {
                            decode_tile_data(data, encoding).map_err(|e| e.to_string())
                        })
                        .and_then(|tiles| {
                            Ok(Chunk {
                                x: chunk.x,
                                y: chunk.y,
                                width: chunk.width,
                                height: chunk.height,
                                tiles,
                            })
                        })
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Values::Chunks),

            _ => Err(String::from("invalid tile data")),
        }
        .map_err(de::Error::custom)?;

        Ok(Data { encoding, values })
    }
}

fn default_opacity() -> f64 {
    1.0
}

#[derive(Debug, Deserialize)]
pub struct Layer {
    pub id: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    pub width: i32,
    pub height: i32,
    #[serde(default)]
    pub locked: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(default, rename = "offsetx")]
    pub offset_x: f64,
    #[serde(default, rename = "offsety")]
    pub offset_y: f64,
    pub data: Data,
}
