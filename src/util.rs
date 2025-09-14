use quick_xml::{
    escape::{escape, resolve_predefined_entity},
    events::{attributes::Attribute, Event},
    name::QName,
    Reader,
};

use crate::error::{Error, XmlError};
use std::borrow::Cow;
use std::io::BufRead;
use std::str::FromStr;

/// Alias of `::chrono::DateTime<::chrono::FixedOffset>`
pub type FixedDateTime = ::chrono::DateTime<::chrono::FixedOffset>;

pub fn default_fixed_datetime() -> FixedDateTime {
    FixedDateTime::from_str("1970-01-01T00:00:00Z").unwrap()
}

pub(crate) fn decode<'s, 'r, B: BufRead>(
    bytes: &'s [u8],
    reader: &'r Reader<B>,
) -> Result<Cow<'s, str>, Error> {
    let text = reader.decoder().decode(bytes).map_err(XmlError::new)?;
    Ok(text)
}

pub(crate) fn attr_value<'s, 'r, B: BufRead>(
    attr: &'s Attribute<'s>,
    reader: &'r Reader<B>,
) -> Result<Cow<'s, str>, Error> {
    let value = attr
        .decode_and_unescape_value(reader.decoder())
        .map_err(XmlError::new)?;
    Ok(value)
}

pub(crate) fn skip<B: BufRead>(end: QName<'_>, reader: &mut Reader<B>) -> Result<(), Error> {
    reader
        .read_to_end_into(end, &mut Vec::new())
        .map_err(XmlError::new)?;
    Ok(())
}

fn non_empty(string: String) -> Option<String> {
    if !string.is_empty() {
        Some(string)
    } else {
        None
    }
}

pub fn atom_text<B: BufRead>(reader: &mut Reader<B>) -> Result<Option<String>, Error> {
    reader.config_mut().expand_empty_elements = false;

    let mut innerbuf = Vec::new();
    let mut depth = 0;
    let mut result = String::new();

    loop {
        match reader
            .read_event_into(&mut innerbuf)
            .map_err(XmlError::new)?
        {
            Event::Start(start) => {
                depth += 1;
                result.push('<');
                result.push_str(decode(&start, reader)?.as_ref());
                result.push('>');
            }
            Event::End(end) => {
                if depth <= 0 {
                    break;
                }
                depth -= 1;
                result.push_str("</");
                result.push_str(decode(end.name().as_ref(), reader)?.as_ref());
                result.push('>');
            }
            Event::Empty(start) => {
                result.push('<');
                result.push_str(decode(&start, reader)?.as_ref());
                result.push_str("/>");
            }
            Event::CData(text) => {
                result.push_str(decode(&text, reader)?.as_ref());
            }
            Event::Text(text) => {
                let decoded = text.decode().map_err(XmlError::new)?;
                result.push_str(&decoded);
            }
            Event::GeneralRef(gref) => {
                let entity = gref.decode().map_err(XmlError::new)?;
                if let Some(resolved_entity) = resolve_predefined_entity(&entity) {
                    result.push_str(resolved_entity);
                } else if let Some(ch) = gref.resolve_char_ref().map_err(XmlError::new)? {
                    result.push(ch);
                } else {
                    result.push('&');
                    result.push_str(&entity);
                    result.push(';');
                }
            }
            Event::Comment(text) => {
                let decoded = text.decode().map_err(XmlError::new)?;
                result.push_str("<!--");
                result.push_str(&decoded);
                result.push_str("-->");
            }
            Event::Decl(_decl) => {}
            Event::PI(_text) => {}
            Event::DocType(_text) => {}
            Event::Eof => return Err(Error::Eof),
        }

        innerbuf.clear();
    }

    reader.config_mut().expand_empty_elements = true;

    Ok(non_empty(result))
}

pub fn atom_xhtml<B: BufRead>(reader: &mut Reader<B>) -> Result<Option<String>, Error> {
    reader.config_mut().expand_empty_elements = false;

    let mut innerbuf = Vec::new();
    let mut depth = 0;
    let mut result = String::new();

    loop {
        match reader
            .read_event_into(&mut innerbuf)
            .map_err(XmlError::new)?
        {
            Event::Start(start) => {
                depth += 1;
                result.push('<');
                result.push_str(decode(&start, reader)?.as_ref());
                result.push('>');
            }
            Event::End(end) => {
                if depth <= 0 {
                    break;
                }
                depth -= 1;
                result.push_str("</");
                result.push_str(decode(end.name().as_ref(), reader)?.as_ref());
                result.push('>');
            }
            Event::Empty(start) => {
                result.push('<');
                result.push_str(decode(&start, reader)?.as_ref());
                result.push_str("/>");
            }
            Event::CData(text) => {
                result.push_str(escape(decode(&text, reader)?.as_ref()).as_ref());
            }
            Event::Text(text) => {
                let decoded = text.decode().map_err(XmlError::new)?;
                result.push_str(escape(decoded.as_ref()).as_ref());
            }
            Event::GeneralRef(gref) => {
                let entity = gref.decode().map_err(XmlError::new)?;
                result.push('&');
                result.push_str(&entity);
                result.push(';');
            }
            Event::Comment(text) => {
                let decoded = text.decode().map_err(XmlError::new)?;
                result.push_str("<!--");
                result.push_str(&decoded);
                result.push_str("-->");
            }
            Event::Decl(_decl) => {}
            Event::PI(_text) => {}
            Event::DocType(_text) => {}
            Event::Eof => return Err(Error::Eof),
        }

        innerbuf.clear();
    }

    reader.config_mut().expand_empty_elements = true;

    Ok(non_empty(result))
}

pub fn atom_datetime<B: BufRead>(reader: &mut Reader<B>) -> Result<Option<FixedDateTime>, Error> {
    if let Some(datetime_text) = atom_text(reader)? {
        match diligent_date_parser::parse_date(&datetime_text) {
            None => Err(Error::WrongDatetime(datetime_text)),
            Some(datetime) => Ok(Some(datetime)),
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;

    fn read_x(xml: &str) -> Result<Option<String>, Error> {
        let mut reader = Reader::from_reader(xml.as_bytes());
        reader.config_mut().expand_empty_elements = true;
        loop {
            let mut buf = Vec::new();
            match reader.read_event_into(&mut buf).map_err(XmlError::new)? {
                Event::Start(element) => {
                    return match decode(element.name().as_ref(), &reader)? {
                        Cow::Borrowed("text") => atom_text(&mut reader),
                        Cow::Borrowed("raw") => atom_xhtml(&mut reader),
                        _ => Err(Error::InvalidStartTag),
                    }
                }
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }
        }
    }

    #[test]
    fn test_read_text() {
        let xml_fragment = r#"<text>
            Text with ampersand &amp; &lt;tag&gt; and <unescaped-empty-tag />.
        </text>"#;
        assert_eq!(
            read_x(xml_fragment).unwrap().unwrap().trim(),
            "Text with ampersand & <tag> and <unescaped-empty-tag />."
        );
    }

    #[test]
    fn test_read_xhtml() {
        let xml_fragment = r#"<raw>
            <div>a line<br/>&amp; one more</div>
        </raw>"#;
        assert_eq!(
            read_x(xml_fragment).unwrap().unwrap().trim(),
            r#"<div>a line<br/>&amp; one more</div>"#
        );
    }
}
