use std::io::BufRead;

use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;

use error::Error;
use fromxml::FromXml;
use util::atom_text;

/// Represents the content of an Atom entry
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Content {
    /// The text value of the content.
    value: Option<String>,
    /// The URI of where the content can be found.
    src: Option<String>,
    /// Either "text", "html", "xhtml", or the MIME type of the content.
    content_type: Option<String>,
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
        self.value.as_ref().map(|s| s.as_str())
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
        where V: Into<Option<String>>
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
        self.src.as_ref().map(|s| s.as_str())
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
        where V: Into<Option<String>>
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
        self.content_type.as_ref().map(|s| s.as_str())
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
        where V: Into<Option<String>>
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

        content.value = atom_text(reader)?;

        Ok(content)
    }
}
