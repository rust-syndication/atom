use std::cmp::PartialEq;
use std::convert::{AsRef, From};
use std::io::{BufRead, Write};
use std::ops::Deref;
use std::str::FromStr;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::{Error, XmlError};
use crate::fromxml::FromXml;
use crate::toxml::ToXmlNamed;
use crate::util::{atom_text, atom_xhtml};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Represents the value of the [`type` attribute of a text construct](https://tools.ietf.org/html/rfc4287#section-3.1.1)
/// in an Atom feed, e.g. the type of the content stored in the element.
pub enum TextType {
    /// Plain text
    Text,
    /// HTML
    Html,
    /// XHTML
    Xhtml,
}

impl Default for TextType {
    fn default() -> Self {
        TextType::Text
    }
}

impl TextType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Html => "html",
            Self::Xhtml => "xhtml",
        }
    }
}

impl FromStr for TextType {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "text" => Ok(Self::Text),
            "html" => Ok(Self::Html),
            "xhtml" => Ok(Self::Xhtml),
            _ => Err(Error::WrongAttribute {
                attribute: "type",
                value: value.to_owned(),
            }),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
/// Represents a [text construct](https://tools.ietf.org/html/rfc4287#section-3.1) in an Atom feed.
pub struct Text {
    /// Content of the text construct
    pub value: String,
    /// Base URL for resolving any relative references found in the element.
    pub base: Option<String>,
    /// Indicates the natural language for the element.
    pub lang: Option<String>,
    /// Type of content stored in the element.
    pub r#type: TextType,
}

impl Text {
    /// Creates a plain text construct (type = "text").
    pub fn plain(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            r#type: TextType::Text,
            ..Self::default()
        }
    }

    /// Creates an html text construct (type = "html").
    pub fn html(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            r#type: TextType::Html,
            ..Self::default()
        }
    }

    /// Creates an html text construct (type = "html").
    pub fn xhtml(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            r#type: TextType::Xhtml,
            ..Self::default()
        }
    }

    /// Returns a content as a `str`
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self::plain(value)
    }
}

impl<'t> From<&'t str> for Text {
    fn from(value: &'t str) -> Self {
        Self::plain(value)
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

        for att in atts.with_checks(false).flatten() {
            match att.key {
                b"xml:base" => {
                    text.base = Some(
                        att.unescape_and_decode_value(reader)
                            .map_err(XmlError::new)?,
                    )
                }
                b"xml:lang" => {
                    text.lang = Some(
                        att.unescape_and_decode_value(reader)
                            .map_err(XmlError::new)?,
                    )
                }
                b"type" => {
                    text.r#type = att
                        .unescape_and_decode_value(reader)
                        .map_err(XmlError::new)?
                        .parse()?
                }
                _ => {}
            }
        }

        let content = if text.r#type == TextType::Xhtml {
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
            element.push_attribute(("xml:base", base.as_str()));
        }
        if let Some(ref lang) = self.lang {
            element.push_attribute(("xml:lang", lang.as_str()));
        }
        if self.r#type != TextType::default() {
            element.push_attribute(("type", self.r#type.as_str()));
        }
        writer.write_event(Event::Start(element)).map_err(XmlError::new)?;
        if self.r#type == TextType::Xhtml {
            writer.write_event(Event::Text(BytesText::from_escaped(self.value.as_bytes()))).map_err(XmlError::new)?;
        } else {
            writer.write_event(Event::Text(BytesText::from_plain_str(self.value.as_str()))).map_err(XmlError::new)?;
        }
        writer.write_event(Event::End(BytesEnd::borrowed(name))).map_err(XmlError::new)?;

        Ok(())
    }
}

#[cfg(feature = "builders")]
impl TextBuilder {
    /// Builds a new `Text`.
    pub fn build(&self) -> Text {
        self.build_impl().unwrap()
    }
}
