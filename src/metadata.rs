use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Metadata {
    /// The TMX format version. Was “1.0” so far, and will be incremented to match minor Tiled releases.
    pub version: String,
    /// The Tiled version used to save the file (since Tiled 1.0.1). May be a date (for snapshot builds).
    #[serde(rename = "tiledversion")]
    pub tiled_version: String,
}
