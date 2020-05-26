#[cfg(feature = "xml")]
#[test]
fn test_xml() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="3" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
      <tile id="0" type="Tile"/>
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_animation() {
    let map = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <map version="1.2" tiledversion="1.3.3" orientation="orthogonal" renderorder="right-down" width="4" height="4" tilewidth="16" tileheight="16" infinite="0" nextlayerid="3" nextobjectid="1">
     <tileset firstgid="1" name="test" tilewidth="16" tileheight="16" tilecount="256" columns="16">
      <image source="tiles16.png" width="256" height="256"/>
      <tile id="0" type="Tile">
       <animation>
        <frame tileid="0" duration="100"/>
        <frame tileid="1" duration="100"/>
       </animation>
      </tile>
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_csv() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(all(feature = "xml", feature = "base64-data"))]
#[test]
fn test_xml_base64() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(all(feature = "xml", feature = "gzip-data"))]
#[test]
fn test_xml_gzip() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(all(feature = "xml", feature = "zlib-data"))]
#[test]
fn test_xml_zlib() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(all(feature = "xml", feature = "zstd-data"))]
#[test]
fn test_xml_zstd() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_chunks() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_chunks_csv() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(all(feature = "xml", feature = "base64-data"))]
#[test]
fn test_xml_chunks_base64() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(all(feature = "xml", feature = "gzip-data"))]
#[test]
fn test_xml_chunks_gzip() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(all(feature = "xml", feature = "zlib-data"))]
#[test]
fn test_xml_chunks_zlib() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[cfg(all(feature = "xml", feature = "zstd-data"))]
#[test]
fn test_xml_chunks_zstd() {
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

    let tmx = tmx::Map::from_xml(map).unwrap();
    println!("xml: {:?}", tmx);
}

#[test]
fn test_json() {
    let map = r##"
    { "compressionlevel":-7,
    "height":4,
    "infinite":false,
    "layers":[
           {
            "data":[1, 2684354561, 1, 2147483649, 1610612737, 3221225473, 1073741825, 3221225473, 2147483649, 3758096385, 1073741825, 536870913, 536870913, 1073741825, 3758096385, 2147483649],
            "height":4,
            "id":1,
            "name":"Tile Layer 1",
            "opacity":1,
            "type":"tilelayer",
            "visible":true,
            "width":4,
            "x":0,
            "y":0
           }],
    "nextlayerid":3,
    "nextobjectid":1,
    "orientation":"orthogonal",
    "renderorder":"right-down",
    "tiledversion":"1.3.3",
    "tileheight":16,
    "tilesets":[
           {
            "columns":16,
            "firstgid":1,
            "image":"tiles16.png",
            "imageheight":256,
            "imagewidth":256,
            "margin":0,
            "name":"test",
            "spacing":0,
            "tilecount":256,
            "tileheight":16,
            "tiles":[
                   {
                    "id":0,
                    "type":"Tile"
                   }],
            "tilewidth":16
           }],
    "tilewidth":16,
    "type":"map",
    "version":1.2,
    "width":4
   }
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[test]
fn test_json_animation() {
    let map = r##"
    { "compressionlevel":-7,
    "height":4,
    "infinite":false,
    "layers":[
           {
            "data":[1, 2684354561, 1, 2147483649, 1610612737, 3221225473, 1073741825, 3221225473, 2147483649, 3758096385, 1073741825, 536870913, 536870913, 1073741825, 3758096385, 2147483649],
            "height":4,
            "id":1,
            "name":"Tile Layer 1",
            "opacity":1,
            "type":"tilelayer",
            "visible":true,
            "width":4,
            "x":0,
            "y":0
           }],
    "nextlayerid":3,
    "nextobjectid":1,
    "orientation":"orthogonal",
    "renderorder":"right-down",
    "tiledversion":"1.3.3",
    "tileheight":16,
    "tilesets":[
           {
            "columns":16,
            "firstgid":1,
            "image":"tiles16.png",
            "imageheight":256,
            "imagewidth":256,
            "margin":0,
            "name":"test",
            "spacing":0,
            "tilecount":256,
            "tileheight":16,
            "tiles":[
                   {
                    "animation":[
                        {
                         "duration":100,
                         "tileid":0
                        }, 
                        {
                         "duration":100,
                         "tileid":1
                        }],
                    "id":0,
                    "type":"Tile"
                   }],
            "tilewidth":16
           }],
    "tilewidth":16,
    "type":"map",
    "version":1.2,
    "width":4
   }
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[cfg(feature = "base64-data")]
#[test]
fn test_json_base64() {
    let map = r##"
    { "compressionlevel":-1,
    "height":4,
    "infinite":false,
    "layers":[
           {
            "data":"AQAAAAEAAKABAAAAAQAAgAEAAGABAADAAQAAQAEAAMABAACAAQAA4AEAAEABAAAgAQAAIAEAAEABAADgAQAAgA==",
            "encoding":"base64",
            "height":4,
            "id":1,
            "name":"Tile Layer 1",
            "opacity":1,
            "type":"tilelayer",
            "visible":true,
            "width":4,
            "x":0,
            "y":0
           }],
    "nextlayerid":2,
    "nextobjectid":1,
    "orientation":"orthogonal",
    "renderorder":"right-down",
    "tiledversion":"1.3.3",
    "tileheight":16,
    "tilesets":[
           {
            "columns":16,
            "firstgid":1,
            "image":"tiles16.png",
            "imageheight":256,
            "imagewidth":256,
            "margin":0,
            "name":"test",
            "spacing":0,
            "tilecount":256,
            "tileheight":16,
            "tilewidth":16
           }],
    "tilewidth":16,
    "type":"map",
    "version":1.2,
    "width":4
   }
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[cfg(feature = "gzip-data")]
#[test]
fn test_json_gzip() {
    let map = r##"
    { "compressionlevel":-1,
    "height":4,
    "infinite":false,
    "layers":[
           {
            "compression":"gzip",
            "data":"H4sIAAAAAAAAE2NkYGBgZGBYwAihG4A4AYgPALEDlAaJPYDyFaDYASrWAAB8ZFU/QAAAAA==",
            "encoding":"base64",
            "height":4,
            "id":1,
            "name":"Tile Layer 1",
            "opacity":1,
            "type":"tilelayer",
            "visible":true,
            "width":4,
            "x":0,
            "y":0
           }],
    "nextlayerid":2,
    "nextobjectid":1,
    "orientation":"orthogonal",
    "renderorder":"right-down",
    "tiledversion":"1.3.3",
    "tileheight":16,
    "tilesets":[
           {
            "columns":16,
            "firstgid":1,
            "image":"tiles16.png",
            "imageheight":256,
            "imagewidth":256,
            "margin":0,
            "name":"test",
            "spacing":0,
            "tilecount":256,
            "tileheight":16,
            "tilewidth":16
           }],
    "tilewidth":16,
    "type":"map",
    "version":1.2,
    "width":4
   }
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[cfg(feature = "zlib-data")]
#[test]
fn test_json_zlib() {
    let map = r##"
    { "compressionlevel":-1,
    "height":4,
    "infinite":false,
    "layers":[
           {
            "compression":"zlib",
            "data":"eJxjZGBgYGRgWMAIoRuAOAGIDwCxA5QGiT2A8hWg2AEq1gAAxKAG0Q==",
            "encoding":"base64",
            "height":4,
            "id":1,
            "name":"Tile Layer 1",
            "opacity":1,
            "type":"tilelayer",
            "visible":true,
            "width":4,
            "x":0,
            "y":0
           }],
    "nextlayerid":2,
    "nextobjectid":1,
    "orientation":"orthogonal",
    "renderorder":"right-down",
    "tiledversion":"1.3.3",
    "tileheight":16,
    "tilesets":[
           {
            "columns":16,
            "firstgid":1,
            "image":"tiles16.png",
            "imageheight":256,
            "imagewidth":256,
            "margin":0,
            "name":"test",
            "spacing":0,
            "tilecount":256,
            "tileheight":16,
            "tilewidth":16
           }],
    "tilewidth":16,
    "type":"map",
    "version":1.2,
    "width":4
   }
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[cfg(feature = "zstd-data")]
#[test]
fn test_json_zstd() {
    let map = r##"
    { "compressionlevel":-1,
    "height":4,
    "infinite":false,
    "layers":[
           {
            "compression":"zstd",
            "data":"KLUv/SBAVQEAyAEAAAABAACggAEAAGABAADAAQAAQOAgQIAGADez7PLNTL5pLZD/ssIF",
            "encoding":"base64",
            "height":4,
            "id":1,
            "name":"Tile Layer 1",
            "opacity":1,
            "type":"tilelayer",
            "visible":true,
            "width":4,
            "x":0,
            "y":0
           }],
    "nextlayerid":2,
    "nextobjectid":1,
    "orientation":"orthogonal",
    "renderorder":"right-down",
    "tiledversion":"1.3.3",
    "tileheight":16,
    "tilesets":[
           {
            "columns":16,
            "firstgid":1,
            "image":"tiles16.png",
            "imageheight":256,
            "imagewidth":256,
            "margin":0,
            "name":"test",
            "spacing":0,
            "tilecount":256,
            "tileheight":16,
            "tilewidth":16
           }],
    "tilewidth":16,
    "type":"map",
    "version":1.2,
    "width":4
   }
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[test]
fn test_json_chunks() {
    let map = r##"
    { "compressionlevel":-7,
 "editorsettings":
    {
     "export":
        {
         "target":"."
        }
    },
 "height":4,
 "infinite":true,
 "layers":[
        {
         "chunks":[
                {
                 "data":[1, 2684354561, 1, 2147483649, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1610612737, 3221225473, 1073741825, 3221225473, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2147483649, 3758096385, 1073741825, 536870913, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 536870913, 1073741825, 3758096385, 2147483649, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                 "height":16,
                 "width":16,
                 "x":0,
                 "y":0
                }],
         "height":16,
         "id":1,
         "name":"Tile Layer 1",
         "opacity":1,
         "startx":0,
         "starty":0,
         "type":"tilelayer",
         "visible":true,
         "width":16,
         "x":0,
         "y":0
        }],
 "nextlayerid":2,
 "nextobjectid":1,
 "orientation":"orthogonal",
 "renderorder":"right-down",
 "tiledversion":"1.3.3",
 "tileheight":16,
 "tilesets":[
        {
         "columns":16,
         "firstgid":1,
         "image":"tiles16.png",
         "imageheight":256,
         "imagewidth":256,
         "margin":0,
         "name":"test",
         "spacing":0,
         "tilecount":256,
         "tileheight":16,
         "tilewidth":16
        }],
 "tilewidth":16,
 "type":"map",
 "version":1.2,
 "width":4
}
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[cfg(feature = "base64-data")]
#[test]
fn test_json_chunks_base64() {
    let map = r##"
    { "compressionlevel":-7,
 "editorsettings":
    {
     "export":
        {
         "target":"."
        }
    },
 "height":4,
 "infinite":true,
 "layers":[
        {
         "chunks":[
                {
                 "data":"AQAAAAEAAKABAAAAAQAAgAEAAGABAADAAQAAQAEAAMABAACAAQAA4AEAAEABAAAgAQAAIAEAAEABAADgAQAAgA==",
                 "height":16,
                 "width":16,
                 "x":0,
                 "y":0
                }],
         "encoding":"base64",
         "height":16,
         "id":1,
         "name":"Tile Layer 1",
         "opacity":1,
         "startx":0,
         "starty":0,
         "type":"tilelayer",
         "visible":true,
         "width":16,
         "x":0,
         "y":0
        }],
 "nextlayerid":2,
 "nextobjectid":1,
 "orientation":"orthogonal",
 "renderorder":"right-down",
 "tiledversion":"1.3.3",
 "tileheight":16,
 "tilesets":[
        {
         "columns":16,
         "firstgid":1,
         "image":"tiles16.png",
         "imageheight":256,
         "imagewidth":256,
         "margin":0,
         "name":"test",
         "spacing":0,
         "tilecount":256,
         "tileheight":16,
         "tilewidth":16
        }],
 "tilewidth":16,
 "type":"map",
 "version":1.2,
 "width":4
}
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[cfg(feature = "gzip-data")]
#[test]
fn test_json_chunks_gzip() {
    let map = r##"
    { "compressionlevel":-7,
 "editorsettings":
    {
     "export":
        {
         "target":"."
        }
    },
 "height":4,
 "infinite":true,
 "layers":[
        {
         "chunks":[
                {
                 "data":"H4sIAAAAAAAAE2NkYGBgZGBYwAihG4A4AYgPALEDlAaJPYDyFaDYASrWAAB8ZFU/QAAAAA==",
                 "height":16,
                 "width":16,
                 "x":0,
                 "y":0
                }],
         "compression":"gzip",
         "encoding":"base64",
         "height":16,
         "id":1,
         "name":"Tile Layer 1",
         "opacity":1,
         "startx":0,
         "starty":0,
         "type":"tilelayer",
         "visible":true,
         "width":16,
         "x":0,
         "y":0
        }],
 "nextlayerid":2,
 "nextobjectid":1,
 "orientation":"orthogonal",
 "renderorder":"right-down",
 "tiledversion":"1.3.3",
 "tileheight":16,
 "tilesets":[
        {
         "columns":16,
         "firstgid":1,
         "image":"tiles16.png",
         "imageheight":256,
         "imagewidth":256,
         "margin":0,
         "name":"test",
         "spacing":0,
         "tilecount":256,
         "tileheight":16,
         "tilewidth":16
        }],
 "tilewidth":16,
 "type":"map",
 "version":1.2,
 "width":4
}
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[cfg(feature = "zlib-data")]
#[test]
fn test_json_chunks_zlib() {
    let map = r##"
    { "compressionlevel":-7,
 "editorsettings":
    {
     "export":
        {
         "target":"."
        }
    },
 "height":4,
 "infinite":true,
 "layers":[
        {
         "chunks":[
                {
                 "data":"eJxjZGBgYGRgWMAIoRuAOAGIDwCxA5QGiT2A8hWg2AEq1gAAxKAG0Q==",
                 "height":16,
                 "width":16,
                 "x":0,
                 "y":0
                }],
         "compression":"zlib",
         "encoding":"base64",
         "height":16,
         "id":1,
         "name":"Tile Layer 1",
         "opacity":1,
         "startx":0,
         "starty":0,
         "type":"tilelayer",
         "visible":true,
         "width":16,
         "x":0,
         "y":0
        }],
 "nextlayerid":2,
 "nextobjectid":1,
 "orientation":"orthogonal",
 "renderorder":"right-down",
 "tiledversion":"1.3.3",
 "tileheight":16,
 "tilesets":[
        {
         "columns":16,
         "firstgid":1,
         "image":"tiles16.png",
         "imageheight":256,
         "imagewidth":256,
         "margin":0,
         "name":"test",
         "spacing":0,
         "tilecount":256,
         "tileheight":16,
         "tilewidth":16
        }],
 "tilewidth":16,
 "type":"map",
 "version":1.2,
 "width":4
}
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[cfg(feature = "zstd-data")]
#[test]
fn test_json_chunks_zstd() {
    let map = r##"
    { "compressionlevel":-7,
 "editorsettings":
    {
     "export":
        {
         "target":"."
        }
    },
 "height":4,
 "infinite":true,
 "layers":[
        {
         "chunks":[
                {
                 "data":"KLUv/SBAVQEAyAEAAAABAACggAEAAGABAADAAQAAQOAgQIAGADez7PLNTL5pLZD/ssIF",
                 "height":16,
                 "width":16,
                 "x":0,
                 "y":0
                }],
         "compression":"zstd",
         "encoding":"base64",
         "height":16,
         "id":1,
         "name":"Tile Layer 1",
         "opacity":1,
         "startx":0,
         "starty":0,
         "type":"tilelayer",
         "visible":true,
         "width":16,
         "x":0,
         "y":0
        }],
 "nextlayerid":2,
 "nextobjectid":1,
 "orientation":"orthogonal",
 "renderorder":"right-down",
 "tiledversion":"1.3.3",
 "tileheight":16,
 "tilesets":[
        {
         "columns":16,
         "firstgid":1,
         "image":"tiles16.png",
         "imageheight":256,
         "imagewidth":256,
         "margin":0,
         "name":"test",
         "spacing":0,
         "tilecount":256,
         "tileheight":16,
         "tilewidth":16
        }],
 "tilewidth":16,
 "type":"map",
 "version":1.2,
 "width":4
}
    "##;

    let tmx = tmx::Map::from_json(map).unwrap();
    println!("json: {:?}", tmx);
}

#[test]
fn test_xml_tileset() {
    let tileset = r##"
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

    let tileset = tmx::Tileset::from_xml(tileset).unwrap();
    println!("tileset: {:?}", tileset);
}

#[test]
fn test_json_tileset() {
    let tileset = r##"
    { "columns":16,
    "image":"tiles16.png",
    "imageheight":256,
    "imagewidth":256,
    "margin":0,
    "name":"tiles16",
    "spacing":0,
    "tilecount":256,
    "tiledversion":"1.3.3",
    "tileheight":16,
    "tiles":[
           {
            "id":0,
            "type":"Solid"
           }, 
           {
            "id":1,
            "type":"Solid"
           }, 
           {
            "id":2,
            "type":"Solid"
           }, 
           {
            "id":3,
            "type":"OneWay"
           }, 
           {
            "id":4,
            "type":"OneWay"
           }, 
           {
            "id":5,
            "type":"OneWay"
           }, 
           {
            "id":16,
            "type":"Solid"
           }, 
           {
            "id":17,
            "type":"Solid"
           }, 
           {
            "id":18,
            "type":"Solid"
           }, 
           {
            "id":19,
            "type":"OneWay"
           }, 
           {
            "id":32,
            "type":"Solid"
           }, 
           {
            "id":33,
            "type":"Solid"
           }, 
           {
            "id":34,
            "type":"Solid"
           }, 
           {
            "id":35,
            "type":"Solid"
           }, 
           {
            "id":36,
            "type":"Solid"
           }, 
           {
            "id":37,
            "type":"Solid"
           }],
    "tilewidth":16,
    "type":"tileset",
    "version":1.2
   }
    "##;

    let tileset = tmx::Tileset::from_json(tileset).unwrap();
    println!("tileset: {:?}", tileset);
}
