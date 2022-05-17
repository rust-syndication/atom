use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::error::{Error, XmlError};
use std::str::FromStr;

/// Alias of `::chrono::DateTime<::chrono::FixedOffset>`
pub type FixedDateTime = ::chrono::DateTime<::chrono::FixedOffset>;

pub fn default_fixed_datetime() -> FixedDateTime {
    FixedDateTime::from_str("1970-01-01T00:00:00Z").unwrap()
}

fn non_empty(string: String) -> Option<String> {
    if !string.is_empty() {
        Some(string)
    } else {
        None
    }
}

pub fn atom_text<B: BufRead>(reader: &mut Reader<B>) -> Result<Option<String>, Error> {
    reader.expand_empty_elements(false);

    let mut innerbuf = Vec::new();
    let mut depth = 0;
    let mut result = String::new();

    loop {
        match reader.read_event(&mut innerbuf).map_err(XmlError::new)? {
            Event::Start(start) => {
                depth += 1;
                result.push('<');
                result.push_str(&start.unescape_and_decode(reader).map_err(XmlError::new)?);
                result.push('>');
            }
            Event::End(end) => {
                if depth <= 0 {
                    break;
                }
                depth -= 1;
                result.push_str("</");
                result.push_str(&reader.decode(end.name()));
                result.push('>');
            }
            Event::Empty(start) => {
                result.push('<');
                result.push_str(&start.unescape_and_decode(reader).map_err(XmlError::new)?);
                result.push_str("/>");
            }
            Event::CData(text) => {
                let decoded = text.unescape_and_decode(reader).map_err(XmlError::new)?;
                result.push_str(&decoded);
            }
            Event::Text(text) => {
                let decoded = text.unescape_and_decode(reader).map_err(XmlError::new)?;
                result.push_str(&decoded);
            }
            Event::Comment(text) => {
                let decoded = text.unescape_and_decode(reader).map_err(XmlError::new)?;
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

    reader.expand_empty_elements(true);

    Ok(non_empty(result))
}

pub fn atom_xhtml<B: BufRead>(reader: &mut Reader<B>) -> Result<Option<String>, Error> {
    reader.expand_empty_elements(false);

    let mut innerbuf = Vec::new();
    let mut depth = 0;
    let mut result = String::new();

    loop {
        match reader.read_event(&mut innerbuf).map_err(XmlError::new)? {
            Event::Start(start) => {
                depth += 1;
                result.push('<');
                result.push_str(&start.unescape_and_decode(reader).map_err(XmlError::new)?);
                result.push('>');
            }
            Event::End(end) => {
                if depth <= 0 {
                    break;
                }
                depth -= 1;
                result.push_str("</");
                result.push_str(&reader.decode(end.name()));
                result.push('>');
            }
            Event::Empty(start) => {
                result.push('<');
                result.push_str(&start.unescape_and_decode(reader).map_err(XmlError::new)?);
                result.push_str("/>");
            }
            Event::CData(text) => {
                let decoded = reader.decode(text.escaped());
                result.push_str(&decoded);
            }
            Event::Text(text) => {
                let decoded = reader.decode(text.escaped());
                result.push_str(&decoded);
            }
            Event::Comment(text) => {
                let decoded = text.unescape_and_decode(reader).map_err(XmlError::new)?;
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

    reader.expand_empty_elements(true);

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
        reader.expand_empty_elements(true);
        loop {
            let mut buf = Vec::new();
            match reader.read_event(&mut buf).map_err(XmlError::new)? {
                Event::Start(element) if element.name() == b"text" => {
                    return atom_text(&mut reader)
                }
                Event::Start(element) if element.name() == b"raw" => {
                    return atom_xhtml(&mut reader)
                }
                Event::Start(_) => return Err(Error::InvalidStartTag),
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
