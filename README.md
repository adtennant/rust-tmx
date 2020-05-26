# TMX

[![Build Status](https://api.travis-ci.org/adtennant/rust-tmx.svg?branch=master)](https://travis-ci.org/adtennant/rust-tmx)
[![Crates.io](https://img.shields.io/crates/v/tmx.svg)](https://crates.io/crates/tmx)
[![Docs](https://docs.rs/tmx/badge.svg)](https://docs.rs/tmx)
[![License](https://img.shields.io/crates/l/tmx.svg)](https://github.com/adtennant/rust-tmx/blob/master/LICENSE)

TMX is a library for loading [Tiled](https://mapeditor.org) (XML and JSON) maps in Rust.

## Usage

```bash
cargo add tmx
```

## Examples

### Loading a Map

```rust
use tmx::Map;

fn main() -> Result<(), Box<dyn Error>> {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="isometric" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="csv">
    1,2684354561,1,2147483649,
    1610612737,3221225473,1073741825,3221225473,
    2147483649,3758096385,1073741825,536870913,
    536870913,1073741825,3758096385,2147483649
    </data>
     </layer>
    </map>
    "##;

    let map = Map::from_xml(map)?;
    println!("{:?}", map);

    Ok(())
}
```

### Loading a Tileset

```rust
use tmx::Tileset;

fn main() -> Result<(), Box<dyn Error>> {
    let tileset = r##"
    <?xml version="1.0" encoding="UTF-8" ?>
<tileset version="1.2" tiledversion="1.3.3" name="tiles16" tilewidth="16" tileheight="16" tilecount="256" columns="16">
    <image source="tiles16.png" width="256" height="256" />
    <tile id="0" type="Solid" />
    <tile id="1" type="Solid" />
    <tile id="2" type="Solid" />
    <tile id="3" type="OneWay" />
</tileset>
    "##;

    let tileset = Tileset::from_xml(tileset)?;
    println!("{:?}", tileset);

    Ok(())
}
```

See the [docs](https://docs.rs/tmx) for more information.

## TMX Map Format Support

| Element            | Support    |
| ------------------ | ---------- |
| `<map>`            | ✅ Full    |
| `<editorsettings>` | 🛑 None    |
| - `<chunksize>`    | 🛑 None    |
| - `<export>`       | 🛑 None    |
| `<tileset>`        | ⚠️ Partial |
| - `<tileoffset>`   | 🛑 None    |
| - `<grid>`         | 🛑 None    |
| - `<image>`        | ✅ Full    |
| - `<terraintypes>` | 🛑 None    |
| - - `<terrain>`    | 🛑 None    |
| - `<tile>`         | ✅ Full    |
| - - `<animation>`  | ✅ Full    |
| - `<wangsets>`     | 🛑 None    |
| - - `<wangset>`    | 🛑 None    |
| `<layer>`          | ✅ Full    |
| - `<data>`         | ✅ Full    |
| - `<chunk>`        | ✅ Full    |
| - `<tile>`         | ✅ Full    |
| `<objectgroup>`    | 🛑 None    |
| - `<object>`       | 🛑 None    |
| - `<ellipse>`      | 🛑 None    |
| - `<point>`        | 🛑 None    |
| - `<polygon>`      | 🛑 None    |
| - `<polyline>`     | 🛑 None    |
| - `<text>`         | 🛑 None    |
| `<imagelayer>`     | 🛑 None    |
| `<group>`          | 🛑 None    |
| `<properties>`     | 🛑 None    |
| - `<property>`     | 🛑 None    |

## Features

The following features are available and enabled by default.

| Feature       | Description                                                                         |
| ------------- | ----------------------------------------------------------------------------------- |
| `xml`         | Allows loading XML maps.                                                            |
| `base64-data` | Allows loading maps where the Tile Layer Format is `Base64 (uncompressed)`.         |
| `gzip-data`   | Allows loading maps where the Tile Layer Format is `Base64 (gzip compressed)`.      |
| `zlib-data`   | Allows loading maps where the Tile Layer Format is `Base64 (zlib compressed)`.      |
| `zstd-data`   | Allows loading maps where the Tile Layer Format is `Base64 (Zstandard compressed)`. |

## License

[MIT](https://github.com/adtennant/rust-tmx/blob/master/LICENSE)
