use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::events::attributes::{Attribute, Attributes};
use quick_xml::Reader;

use error::Error;

fn non_empty(string: String) -> Option<String> {
    if !string.is_empty() {
        Some(string)
    } else {
        None
    }
}

pub fn atom_text<B: BufRead>(reader: &mut Reader<B>) -> Result<Option<String>, Error> {
    let mut innerbuf = Vec::new();
    let mut depth = 0;
    let mut result = String::new();

    loop {
        match reader.read_event(&mut innerbuf)? {
            Event::Start(start) => {
                depth += 1;
                result.push('<');
                result.push_str(&start.unescape_and_decode(reader)?);
                result.push('>');
            },
            Event::End(end) => {
                if depth <= 0 {
                    break;
                }
                depth -= 1;
                result.push_str("</");
                result.push_str(&reader.decode(end.name()));
                result.push('>');
            },
            Event::Empty(start) => {
                depth += 1;
                result.push('<');
                result.push_str(&start.unescape_and_decode(reader)?);
                result.push_str("/>");
            },
            Event::CData(text) => {
                let decoded = reader.decode(text.escaped());
                result.push_str(&decoded);
            },
            Event::Text(text) => {
                let decoded = text.unescape_and_decode(reader)?;
                result.push_str(&decoded);
            },
            Event::Comment(text) => {
                let decoded = text.unescape_and_decode(reader)?;
                result.push_str("<!--");
                result.push_str(&decoded);
                result.push_str("-->");
            },
            Event::Decl(_decl) => {
            },
            Event::PI(_text) => {
            },
            Event::DocType(_text) => {
            },
            Event::Eof => return Err(Error::Eof)
        }

        innerbuf.clear();
    }

    Ok(non_empty(result))
}

pub fn atom_xhtml<B: BufRead>(reader: &mut Reader<B>) -> Result<Option<String>, Error> {
    let mut innerbuf = Vec::new();
    let mut depth = 0;
    let mut result = String::new();

    loop {
        match reader.read_event(&mut innerbuf)? {
            Event::Start(start) => {
                depth += 1;
                result.push('<');
                result.push_str(&start.unescape_and_decode(reader)?);
                result.push('>');
            },
            Event::End(end) => {
                if depth <= 0 {
                    break;
                }
                depth -= 1;
                result.push_str("</");
                result.push_str(&reader.decode(end.name()));
                result.push('>');
            },
            Event::Empty(start) => {
                depth += 1;
                result.push('<');
                result.push_str(&start.unescape_and_decode(reader)?);
                result.push_str("/>");
            },
            Event::CData(text) => {
                let decoded = reader.decode(text.escaped());
                result.push_str(&decoded);
            },
            Event::Text(text) => {
                let decoded = reader.decode(text.escaped());
                result.push_str(&decoded);
            },
            Event::Comment(text) => {
                let decoded = text.unescape_and_decode(reader)?;
                result.push_str("<!--");
                result.push_str(&decoded);
                result.push_str("-->");
            },
            Event::Decl(_decl) => {
            },
            Event::PI(_text) => {
            },
            Event::DocType(_text) => {
            },
            Event::Eof => return Err(Error::Eof)
        }

        innerbuf.clear();
    }

    Ok(non_empty(result))
}

pub fn atom_any_text<B: BufRead>(reader: &mut Reader<B>, mut atts: Attributes) -> Result<Option<String>, Error> {
    let mut content_type = None;
    for attr in atts.with_checks(false) {
        if let Ok(att@Attribute { key: b"type", .. }) = attr {
            content_type = Some(att.unescape_and_decode_value(reader)?);
        }
    }

    match content_type {
        Some(ref t) if t == "xhtml" => atom_xhtml(reader),
        _ => atom_text(reader)
    }
}
