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
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct Content {
    /// Base URL for resolving any relative references found in the element.
    pub base: Option<String>,
    /// Indicates the natural language for the element.
    pub lang: Option<String>,
    /// The text value of the content.
    pub value: Option<String>,
    /// The URI of where the content can be found.
    pub src: Option<String>,
    /// Either "text", "html", "xhtml", or the MIME type of the content.
    pub content_type: Option<String>,
}

impl Content {
    /// Return base URL of the content.
    pub fn base(&self) -> Option<&str> {
        self.base.as_deref()
    }

    /// Set base URL of the content.
    pub fn set_base<V>(&mut self, base: V)
    where
        V: Into<Option<String>>,
    {
        self.base = base.into();
    }

    /// Return natural language of the content.
    pub fn lang(&self) -> Option<&str> {
        self.lang.as_deref()
    }

    /// Set the base URL of the content.
    pub fn set_lang<V>(&mut self, lang: V)
    where
        V: Into<Option<String>>,
    {
        self.lang = lang.into();
    }

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
        self.value.as_deref()
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
        self.src.as_deref()
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
        self.content_type.as_deref()
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
    fn from_xml<B: BufRead>(
        reader: &mut Reader<B>,
        mut atts: Attributes<'_>,
    ) -> Result<Self, Error> {
        let mut content = Content::default();

        for att in atts.with_checks(false).flatten() {
            match att.key {
                b"xml:base" => content.base = Some(att.unescape_and_decode_value(reader)?),
                b"xml:lang" => content.lang = Some(att.unescape_and_decode_value(reader)?),
                b"type" => content.content_type = Some(att.unescape_and_decode_value(reader)?),
                b"src" => content.src = Some(att.unescape_and_decode_value(reader)?),
                _ => {}
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

        if let Some(ref base) = self.base {
            element.push_attribute(("xml:base", base.as_str()));
        }

        if let Some(ref lang) = self.lang {
            element.push_attribute(("xml:lang", lang.as_str()));
        }

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
            writer.write_event(Event::Text(
                if self.content_type.as_deref() == Some("xhtml") {
                    BytesText::from_escaped(value.as_bytes())
                } else {
                    BytesText::from_plain(value.as_bytes())
                },
            ))?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;

        Ok(())
    }
}

#[cfg(feature = "builders")]
impl ContentBuilder {
    /// Builds a new `Content`.
    pub fn build(&self) -> Content {
        self.build_impl().unwrap()
    }
}
