use serde::{de::Deserializer, Deserialize};

fn deserialize_version<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        String(String),
        Float(f64),
    }

    match StringOrFloat::deserialize(deserializer)? {
        StringOrFloat::String(s) => Ok(s),
        StringOrFloat::Float(f) => Ok(f.to_string()),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Metadata {
    #[serde(deserialize_with = "deserialize_version")]
    pub version: String,
    #[serde(rename = "tiledversion")]
    pub tiled_version: String,
}
