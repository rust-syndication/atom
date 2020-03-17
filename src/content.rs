use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::Error;
use crate::fromxml::FromXml;
use crate::toxml::ToXml;
use crate::util::atom_any_text;

/// Represents the content of an Atom entry
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(feature = "builders", builder(setter(into), default))]
pub struct Content {
    /// The text value of the content.
    pub value: Option<String>,
    /// The URI of where the content can be found.
    pub src: Option<String>,
    /// Either "text", "html", "xhtml", or the MIME type of the content.
    pub content_type: Option<String>,
}

impl Content {
    /// Return the text value of the content.
    ///
    /// If the `content_type` is neither `"text"`, `"html"`, or `"xhtml"` then the value should
    /// be a base64 encoded document of the indicated MIME type.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Content;
    ///
    /// let mut content = Content::default();
    /// content.set_value("Example content".to_string());
    /// assert_eq!(content.value(), Some("Example content"));
    /// ```
    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(String::as_str)
    }

    /// Set the text value of the content.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Content;
    ///
    /// let mut content = Content::default();
    /// content.set_value("Example content".to_string());
    /// ```
    pub fn set_value<V>(&mut self, value: V)
    where
        V: Into<Option<String>>,
    {
        self.value = value.into();
    }

    /// Return the URI where the content can be found.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Content;
    ///
    /// let mut content = Content::default();
    /// content.set_src("http://example.com/content.html".to_string());
    /// assert_eq!(content.src(), Some("http://example.com/content.html"));
    /// ```
    pub fn src(&self) -> Option<&str> {
        self.src.as_ref().map(String::as_str)
    }

    /// Set the URI where the content can be found.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Content;
    ///
    /// let mut content = Content::default();
    /// content.set_src("http://example.com/content.html".to_string());
    /// ```
    pub fn set_src<V>(&mut self, src: V)
    where
        V: Into<Option<String>>,
    {
        self.src = src.into();
    }

    /// Return the type of the content.
    ///
    /// The type is either `"text"`, `"html"`, `"xhtml"`, or the MIME type of the content.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Content;
    ///
    /// let mut content = Content::default();
    /// content.set_content_type("image/png".to_string());
    /// assert_eq!(content.content_type(), Some("image/png"));
    /// ```
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_ref().map(String::as_str)
    }

    /// Set the type of the content.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Content;
    ///
    /// let mut content = Content::default();
    /// content.set_content_type("image/png".to_string());
    /// assert_eq!(content.content_type(), Some("image/png"));
    /// ```
    pub fn set_content_type<V>(&mut self, content_type: V)
    where
        V: Into<Option<String>>,
    {
        self.content_type = content_type.into();
    }
}

impl FromXml for Content {
    fn from_xml<B: BufRead>(reader: &mut Reader<B>, mut atts: Attributes) -> Result<Self, Error> {
        let mut content = Content::default();

        for attr in atts.with_checks(false) {
            if let Ok(att) = attr {
                match att.key {
                    b"type" => content.content_type = Some(att.unescape_and_decode_value(reader)?),
                    b"src" => content.src = Some(att.unescape_and_decode_value(reader)?),
                    _ => {}
                }
            }
        }

        content.value = atom_any_text(reader, atts)?;

        Ok(content)
    }
}

impl ToXml for Content {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"content";
        let mut element = BytesStart::borrowed(name, name.len());

        if let Some(ref content_type) = self.content_type {
            if content_type == "xhtml" {
                element.push_attribute(("type", "xhtml"));
            } else {
                element.push_attribute(("type", &**content_type));
            }
        }

        if let Some(ref src) = self.src {
            element.push_attribute(("src", &**src));
        }

        writer.write_event(Event::Start(element))?;

        if let Some(ref value) = self.value {
            writer.write_event(Event::Text(BytesText::from_escaped(value.as_bytes())))?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;

        Ok(())
    }
}
