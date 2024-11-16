use std::borrow::Cow;
use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::{Error, XmlError};
use crate::fromxml::FromXml;
use crate::toxml::ToXml;
use crate::util::{atom_text, atom_xhtml, attr_value, decode};

/// Represents the content of an Atom entry
//
/// ## Attention
///
/// Atom format specification [RFC4287](https://datatracker.ietf.org/doc/html/rfc4287#section-4.1.3.2)
/// states that `src` and `value` (content) fields are mutually exclusive:
///
/// > atom:content MAY have a "src" attribute, whose value MUST be an IRI reference.
/// > If the "src" attribute is present, atom:content MUST be empty.
///
/// Setting of both fields when authoring an Atom feed is still technically possible,
/// but it will lead to a non-compliant result.
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
            match decode(att.key.as_ref(), reader)? {
                Cow::Borrowed("xml:base") => {
                    content.base = Some(attr_value(&att, reader)?.to_string());
                }
                Cow::Borrowed("xml:lang") => {
                    content.lang = Some(attr_value(&att, reader)?.to_string());
                }
                Cow::Borrowed("type") => {
                    content.content_type = Some(attr_value(&att, reader)?.to_string());
                }
                Cow::Borrowed("src") => {
                    content.src = Some(attr_value(&att, reader)?.to_string());
                }
                _ => {}
            }
        }

        content.value = match content.content_type {
            Some(ref t) if t == "xhtml" => atom_xhtml(reader)?,
            _ => atom_text(reader)?,
        };

        Ok(content)
    }
}

impl ToXml for Content {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "content";
        let mut element = BytesStart::new(name);

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

        writer
            .write_event(Event::Start(element))
            .map_err(XmlError::new)?;

        if let Some(ref value) = self.value {
            writer
                .write_event(Event::Text(
                    if self.content_type.as_deref() == Some("xhtml") {
                        BytesText::from_escaped(value)
                    } else {
                        BytesText::new(value)
                    },
                ))
                .map_err(XmlError::new)?;
        }

        writer
            .write_event(Event::End(BytesEnd::new(name)))
            .map_err(XmlError::new)?;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;
    use crate::util::decode;

    fn lines(text: &str) -> Vec<&str> {
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
    }

    fn to_xml(content: &Content) -> String {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);
        content.to_xml(&mut writer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    fn from_xml(xml: &str) -> Result<Content, Error> {
        let mut reader = Reader::from_reader(xml.as_bytes());
        reader.config_mut().expand_empty_elements = true;

        loop {
            let mut buf = Vec::new();
            match reader.read_event_into(&mut buf).map_err(XmlError::new)? {
                Event::Start(element) => {
                    if decode(element.name().as_ref(), &reader)? == "content" {
                        let content = Content::from_xml(&mut reader, element.attributes())?;
                        return Ok(content);
                    } else {
                        return Err(Error::InvalidStartTag);
                    }
                }
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }
        }
    }

    #[test]
    fn test_plain_text() {
        let content = Content {
            value: Some("Text with ampersand & <tag>.".into()),
            ..Default::default()
        };
        let xml_fragment = r#"<content>Text with ampersand &amp; &lt;tag&gt;.</content>"#;
        assert_eq!(to_xml(&content), xml_fragment);
        assert_eq!(from_xml(xml_fragment).unwrap(), content);
    }

    #[test]
    fn test_html() {
        let content = Content {
            content_type: Some("html".into()),
            value: Some("Markup with ampersand, <tag>, & </closing-tag>.".into()),
            ..Default::default()
        };
        let xml_fragment = r#"<content type="html">Markup with ampersand, &lt;tag&gt;, &amp; &lt;/closing-tag&gt;.</content>"#;
        assert_eq!(to_xml(&content), xml_fragment);
        assert_eq!(from_xml(xml_fragment).unwrap(), content);
    }

    #[test]
    fn test_xhtml() {
        let content = Content {
            content_type: Some("xhtml".into()),
            value: Some(r#"<div>a line<br/>&amp; one more</div>"#.into()),
            ..Default::default()
        };
        let xml_fragment =
            r#"<content type="xhtml"><div>a line<br/>&amp; one more</div></content>"#;
        assert_eq!(to_xml(&content), xml_fragment);
        assert_eq!(from_xml(xml_fragment).unwrap(), content);
    }

    #[test]
    fn test_write_image() {
        let content = Content {
            content_type: Some("image/png".into()),
            src: Some("http://example.com/image.png".into()),
            ..Default::default()
        };
        assert_eq!(
            lines(&to_xml(&content)),
            lines(
                r#"
                    <content type="image/png" src="http://example.com/image.png">
                    </content>
                "#
            )
        );
    }
}
