use std::collections::BTreeMap;
use std::io::BufRead;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::error::{Error, XmlError};
use crate::extension::{Extension, ExtensionMap};
use crate::util::{attr_value, decode};

pub fn extension_name(element_name: &str) -> Option<(&str, &str)> {
    let mut split = element_name.splitn(2, ':');
    let ns = split.next().filter(|ns| !ns.is_empty())?;
    let name = split.next()?;
    Some((ns, name))
}

pub fn parse_extension<R>(
    reader: &mut Reader<R>,
    atts: Attributes<'_>,
    ns: &str,
    name: &str,
    extensions: &mut ExtensionMap,
) -> Result<(), Error>
where
    R: BufRead,
{
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
        let key = decode(attr.key.local_name().as_ref(), reader)?.to_string();
        let value = attr_value(&attr, reader)?.to_string();
        extension.attrs.insert(key, value);
    }

    let mut text = String::new();
    loop {
        match reader.read_event_into(&mut buf).map_err(XmlError::new)? {
            Event::Start(element) => {
                let ext = parse_extension_element(reader, element.attributes())?;
                let element_local_name = element.local_name();
                let name = decode(element_local_name.as_ref(), reader)?;

                if !extension.children.contains_key(&*name) {
                    extension.children.insert(name.to_string(), Vec::new());
                }

                let items = match extension.children.get_mut(&*name) {
                    Some(items) => items,
                    None => unreachable!(),
                };

                items.push(ext);
            }
            Event::CData(element) => {
                text.push_str(decode(&element, reader)?.as_ref());
            }
            Event::Text(element) => {
                text.push_str(element.unescape().map_err(XmlError::new)?.as_ref());
            }
            Event::End(element) => {
                extension.name = decode(element.name().as_ref(), reader)?.into();
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
