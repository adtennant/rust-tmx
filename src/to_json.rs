use quick_xml::{
    events::{attributes::Attributes, Event},
    Reader,
};
use serde_json::{Map, Value};
use std::io::BufRead;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse string")]
    ParseStringError(#[from] std::string::FromUtf8Error),
    #[error("failed to deserialize XML")]
    XmlError(#[from] quick_xml::Error),
    #[error("unexpected EOF")]
    UnexpectedEof,
    #[error("invalid array")]
    InvalidArray,
}

fn parse_tag<B: BufRead>(
    reader: &mut Reader<B>,
    buf: &mut Vec<u8>,
    root: bool,
) -> Result<Map<String, Value>, Error> {
    let mut children = Map::new();

    loop {
        let event = reader.read_event(buf);

        let mut start_tag =
            |name: &[u8], attributes: Attributes, map: Map<String, Value>| -> Result<(), Error> {
                let mut map = map;

                for attribute in attributes {
                    let attribute = attribute?;
                    map.insert(
                        String::from_utf8(attribute.key.to_vec())?,
                        Value::String(String::from_utf8(attribute.unescaped_value()?.to_vec())?),
                    );
                }

                let key = String::from_utf8(name.to_vec())?;

                match &mut children.get_mut(&key) {
                    None => {
                        children.insert(key, Value::Array(vec![Value::Object(map)]));
                    }
                    Some(value) => {
                        value
                            .as_array_mut()
                            .ok_or(Error::InvalidArray)?
                            .push(Value::Object(map));
                    }
                }

                Ok(())
            };

        match event {
            Ok(Event::Start(ref e)) => {
                let mut buf = vec![];
                start_tag(
                    e.name(),
                    e.attributes(),
                    parse_tag(reader, &mut buf, false)?,
                )?;
            }
            Ok(Event::End(ref _e)) => {
                break;
            }
            Ok(Event::Empty(ref e)) => {
                start_tag(e.name(), e.attributes(), Map::new())?;
            }
            Ok(Event::Text(ref e)) => {
                let string = e.unescape_and_decode(&reader)?;

                if string.trim().is_empty() {
                    continue;
                }

                children.insert("_".to_string(), Value::String(string));
            }
            Ok(Event::Comment(ref _e)) => {}
            Ok(Event::CData(ref _e)) => {}
            Ok(Event::Decl(ref _e)) => {}
            Ok(Event::PI(ref _e)) => {}
            Ok(Event::DocType(ref _e)) => {}
            Ok(Event::Eof) => {
                if root {
                    break;
                }

                return Err(Error::UnexpectedEof);
            }
            Err(e) => return Err(Error::XmlError(e)),
        }

        buf.clear();
    }

    Ok(children)
}

pub fn to_json(xml: &str) -> Result<Value, Error> {
    let mut buf = vec![];
    let mut reader = Reader::from_str(xml);

    Ok(Value::Object(parse_tag(&mut reader, &mut buf, true)?))
}
