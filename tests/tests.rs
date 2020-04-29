use tmx::{Map, Tileset};

#[test]
fn test_orthogonal() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
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

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_orthogonal: {:?}", tmx);
}

#[test]
fn test_isometric() {
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

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_isometric: {:?}", tmx);
}

#[test]
fn test_isometric_staggered() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="staggered" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" staggeraxis="y" staggerindex="odd" nextlayerid="2" nextobjectid="1">
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

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_isometric_staggered: {:?}", tmx);
}

#[test]
fn test_hexagonal() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="hexagonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" hexsidelength="0" staggeraxis="y" staggerindex="odd" nextlayerid="2" nextobjectid="1">
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

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_hexagonal: {:?}", tmx);
}

#[test]
fn test_xml_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data>
       <tile gid="1"/>
       <tile gid="2684354561"/>
       <tile gid="1"/>
       <tile gid="2147483649"/>
       <tile gid="1610612737"/>
       <tile gid="3221225473"/>
       <tile gid="1073741825"/>
       <tile gid="3221225473"/>
       <tile gid="2147483649"/>
       <tile gid="3758096385"/>
       <tile gid="1073741825"/>
       <tile gid="536870913"/>
       <tile gid="536870913"/>
       <tile gid="1073741825"/>
       <tile gid="3758096385"/>
       <tile gid="2147483649"/>
      </data>
     </layer>
    </map>
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_xml_data: {:?}", tmx);
}

#[test]
fn test_csv_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
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

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_csv_data: {:?}", tmx);
}

#[cfg(feature = "base64-data")]
#[test]
fn test_base64_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="base64">
       AQAAAAEAAKABAAAAAQAAgAEAAGABAADAAQAAQAEAAMABAACAAQAA4AEAAEABAAAgAQAAIAEAAEABAADgAQAAgA==
      </data>
     </layer>
    </map>
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_base64_data: {:?}", tmx);
}

#[cfg(feature = "gzip-data")]
#[test]
fn test_gzip_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="base64" compression="gzip">
       H4sIAAAAAAAAE2NkYGBgZGBYwAihG4A4AYgPALEDlAaJPYDyFaDYASrWAAB8ZFU/QAAAAA==
      </data>
     </layer>
    </map>    
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_gzip_data: {:?}", tmx);
}

#[cfg(feature = "zlib-data")]
#[test]
fn test_zlib_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="base64" compression="zlib">
       eJxjZGBgYGRgWMAIoRuAOAGIDwCxA5QGiT2A8hWg2AEq1gAAxKAG0Q==
      </data>
     </layer>
    </map>    
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_zlib_data: {:?}", tmx);
}

#[cfg(feature = "zstd-data")]
#[test]
fn test_zstd_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="base64" compression="zstd">
       KLUv/SBAVQEAyAEAAAABAACggAEAAGABAADAAQAAQOAgQIAGADez7PLNTL5pLZD/ssIF
      </data>
     </layer>
    </map>
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_zstd_data: {:?}", tmx);
}

#[test]
fn test_chunk_xml_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="1" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data>
       <chunk x="0" y="0" width="4" height="4">
        <tile gid="1"/>
        <tile gid="2684354561"/>
        <tile gid="1"/>
        <tile gid="2147483649"/>
        <tile gid="1610612737"/>
        <tile gid="3221225473"/>
        <tile gid="1073741825"/>
        <tile gid="3221225473"/>
        <tile gid="2147483649"/>
        <tile gid="3758096385"/>
        <tile gid="1073741825"/>
        <tile gid="536870913"/>
        <tile gid="536870913"/>
        <tile gid="1073741825"/>
        <tile gid="3758096385"/>
        <tile gid="2147483649"/>
       </chunk>
      </data>
     </layer>
    </map>
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_chunk_xml_data: {:?}", tmx);
}

#[test]
fn test_chunk_csv_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="1" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="csv">
       <chunk x="0" y="0" width="4" height="4">
    1,2684354561,1,2147483649,
    1610612737,3221225473,1073741825,3221225473,
    2147483649,3758096385,1073741825,536870913,
    536870913,1073741825,3758096385,2147483649
    </chunk>
      </data>
     </layer>
    </map>
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_chunk_csv_data: {:?}", tmx);
}

#[cfg(feature = "base64-data")]
#[test]
fn test_chunk_base64_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="1" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="base64">
       <chunk x="0" y="0" width="4" height="4">
       AQAAAAEAAKABAAAAAQAAgAEAAGABAADAAQAAQAEAAMABAACAAQAA4AEAAEABAAAgAQAAIAEAAEABAADgAQAAgA==
      </chunk>
      </data>
     </layer>
    </map>    
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_chunk_base64_data: {:?}", tmx);
}

#[cfg(feature = "gzip-data")]
#[test]
fn test_chunk_gzip_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="1" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="base64" compression="gzip">
       <chunk x="0" y="0" width="4" height="4">
       H4sIAAAAAAAAE2NkYGBgZGBYwAihG4A4AYgPALEDlAaJPYDyFaDYASrWAAB8ZFU/QAAAAA==
      </chunk>
      </data>
     </layer>
    </map>     
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_chunk_gzip_data: {:?}", tmx);
}

#[cfg(feature = "zlib-data")]
#[test]
fn test_chunk_zlib_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="1" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="base64" compression="zlib">
       <chunk x="0" y="0" width="4" height="4">
       eJxjZGBgYGRgWMAIoRuAOAGIDwCxA5QGiT2A8hWg2AEq1gAAxKAG0Q==
      </chunk>
      </data>
     </layer>
    </map>    
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_chunk_zlib_data: {:?}", tmx);
}

#[cfg(feature = "zstd-data")]
#[test]
fn test_chunk_zstd_data() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="1" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data encoding="base64" compression="zstd">
       <chunk x="0" y="0" width="4" height="4">
       KLUv/SBAVQEAyAEAAAABAACggAEAAGABAADAAQAAQOAgQIAGADez7PLNTL5pLZD/ssIF
      </chunk>
      </data>
     </layer>
    </map>    
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();
    println!("test_chunk_zstd_data: {:?}", tmx);
}

#[test]
fn test_gid() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="2" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
     </tileset>
     <layer id="1" name="Tile Layer 1" width="4" height="4">
      <data>
       <tile gid="1"/>
       <tile gid="2684354561"/>
       <tile gid="1"/>
       <tile gid="2147483649"/>
       <tile gid="1610612737"/>
       <tile gid="3221225473"/>
       <tile gid="1073741825"/>
       <tile gid="3221225473"/>
       <tile gid="2147483649"/>
       <tile gid="3758096385"/>
       <tile gid="1073741825"/>
       <tile gid="536870913"/>
       <tile gid="536870913"/>
       <tile gid="1073741825"/>
       <tile gid="3758096385"/>
       <tile gid="2147483649"/>
      </data>
     </layer>
    </map>
    "##;

    let tmx = Map::from_reader(map.as_bytes()).unwrap();

    if let tmx::layer::DataKind::Tiles(tiles) = &tmx.layers[0].data.kind {
        for tile in tiles.iter() {
            println!(
                "{} {} {} {}",
                tile.gid(),
                tile.flipped_horizontally(),
                tile.flipped_vertically(),
                tile.flipped_diagonally()
            );
        }
    }
}

#[cfg(feature = "zstd-data")]
#[test]
fn test_tileset() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8" ?>
    <tileset version="1.2" tiledversion="1.3.3" name="tiles16" tilewidth="16" tileheight="16" tilecount="256" columns="16">
        <image source="tiles16.png" width="256" height="256" />
        <tile id="0" type="Solid" />
        <tile id="1" type="Solid" />
        <tile id="2" type="Solid" />
        <tile id="3" type="OneWay" />
        <tile id="4" type="OneWay" />
        <tile id="5" type="OneWay" />
        <tile id="16" type="Solid" />
        <tile id="17" type="Solid" />
        <tile id="18" type="Solid" />
        <tile id="19" type="OneWay" />
        <tile id="32" type="Solid" />
        <tile id="33" type="Solid" />
        <tile id="34" type="Solid" />
        <tile id="35" type="Solid" />
        <tile id="36" type="Solid" />
        <tile id="37" type="Solid" />
    </tileset>   
    "##;

    let tileset = Tileset::from_reader(map.as_bytes()).unwrap();
    println!("test_tileset: {:?}", tileset);
}
