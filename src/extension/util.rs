use std::collections::BTreeMap;
use std::io::BufRead;
use std::str;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::error::{Error, XmlError};
use crate::extension::{Extension, ExtensionMap};

pub fn extension_name(element_name: &[u8]) -> Option<(&[u8], &[u8])> {
    let mut split = element_name.splitn(2, |b| *b == b':');
    match split.next() {
        Some(b"") | None => None,
        Some(ns) => split.next().map(|name| (ns, name)),
    }
}

pub fn parse_extension<R>(
    reader: &mut Reader<R>,
    atts: Attributes<'_>,
    ns: &[u8],
    name: &[u8],
    extensions: &mut ExtensionMap,
) -> Result<(), Error>
where
    R: BufRead,
{
    let ns = str::from_utf8(ns)?;
    let name = str::from_utf8(name)?;
    let ext = parse_extension_element(reader, atts)?;

    if !extensions.contains_key(ns) {
        extensions.insert(ns.to_string(), BTreeMap::new());
    }

    let map = match extensions.get_mut(ns) {
        Some(map) => map,
        None => unreachable!(),
    };

    if !map.contains_key(name) {
        map.insert(name.to_string(), Vec::new());
    }

    let items = match map.get_mut(name) {
        Some(items) => items,
        None => unreachable!(),
    };

    items.push(ext);

    Ok(())
}

fn parse_extension_element<R: BufRead>(
    reader: &mut Reader<R>,
    mut atts: Attributes<'_>,
) -> Result<Extension, Error> {
    let mut extension = Extension::default();
    let mut buf = Vec::new();

    for attr in atts.with_checks(false).flatten() {
        let key = str::from_utf8(attr.key)?;
        let value = attr
            .unescape_and_decode_value(reader)
            .map_err(XmlError::new)?;
        extension.attrs.insert(key.to_string(), value);
    }

    let mut text = String::new();
    loop {
        match reader.read_event(&mut buf).map_err(XmlError::new)? {
            Event::Start(element) => {
                let ext = parse_extension_element(reader, element.attributes())?;
                let name = str::from_utf8(element.local_name())?;

                if !extension.children.contains_key(name) {
                    extension.children.insert(name.to_string(), Vec::new());
                }

                let items = match extension.children.get_mut(name) {
                    Some(items) => items,
                    None => unreachable!(),
                };

                items.push(ext);
            }
            Event::CData(element) => {
                text.push_str(reader.decode(&element).as_ref());
            }
            Event::Text(element) => {
                text.push_str(
                    element
                        .unescape_and_decode(reader)
                        .map_err(XmlError::new)?
                        .as_ref(),
                );
            }
            Event::End(element) => {
                extension.name = reader.decode(element.name()).into();
                break;
            }
            Event::Eof => return Err(Error::Eof),
            _ => {}
        }

        buf.clear();
    }
    extension.value = Some(text.trim())
        .filter(|t| !t.is_empty())
        .map(ToString::to_string);

    Ok(extension)
}
