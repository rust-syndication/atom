use std::cmp::PartialEq;
use std::convert::{AsRef, From};
use std::io::{BufRead, Write};
use std::ops::Deref;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::Error;
use crate::fromxml::FromXml;
use crate::toxml::ToXmlNamed;
use crate::util::{atom_text, atom_xhtml};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq, Default)]
/// Represents a [text construct](https://tools.ietf.org/html/rfc4287#section-3.1) in an Atom feed.
pub struct Text {
    /// Content of the text construct
    pub value: String,
    /// Base URL for resolving any relative references found in the element.
    pub base: Option<String>,
    /// Indicates the natural language for the element.
    pub lang: Option<String>,
    /// Type of content stored in the element.
    /// When present, the value MUST be one of "text", "html", or "xhtml".
    /// When absent, "text" is assumed.
    pub content_type: Option<String>,
}

impl Text {
    /// Returns a content as a `str`
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text {
            value,
            ..Self::default()
        }
    }
}

impl<'t> From<&'t str> for Text {
    fn from(value: &'t str) -> Self {
        Text {
            value: value.to_owned(),
            ..Self::default()
        }
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<Text> for str {
    fn eq(&self, other: &Text) -> bool {
        self == other.as_str()
    }
}

impl FromXml for Text {
    fn from_xml<B: BufRead>(
        reader: &mut Reader<B>,
        mut atts: Attributes<'_>,
    ) -> Result<Self, Error> {
        let mut text = Text::default();

        for attr in atts.with_checks(false) {
            if let Ok(att) = attr {
                match att.key {
                    b"base" => text.base = Some(att.unescape_and_decode_value(reader)?),
                    b"lang" => text.lang = Some(att.unescape_and_decode_value(reader)?),
                    b"type" => text.content_type = Some(att.unescape_and_decode_value(reader)?),
                    _ => {}
                }
            }
        }

        let content = if text.content_type.as_deref() == Some("xhtml") {
            atom_xhtml(reader)?
        } else {
            atom_text(reader)?
        };

        text.value = content.unwrap_or_default();

        Ok(text)
    }
}

impl ToXmlNamed for Text {
    fn to_xml_named<W, N>(&self, writer: &mut Writer<W>, name: N) -> Result<(), XmlError>
    where
        W: Write,
        N: AsRef<[u8]>,
    {
        let name = name.as_ref();
        let mut element = BytesStart::borrowed(name, name.len());
        if let Some(ref base) = self.base {
            element.push_attribute(("base", base.as_str()));
        }
        if let Some(ref lang) = self.lang {
            element.push_attribute(("lang", lang.as_str()));
        }
        if let Some(ref content_type) = self.content_type {
            element.push_attribute(("type", content_type.as_str()));
        }
        writer.write_event(Event::Start(element))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(self.value.as_str())))?;
        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;

        Ok(())
    }
}
