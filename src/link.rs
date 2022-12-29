use std::borrow::Cow;
use std::io::{BufRead, Write};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::{Error, XmlError};
use crate::toxml::ToXml;
use crate::util::{attr_value, decode};

/// Represents a link in an Atom feed
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct Link {
    /// The URI of the referenced resource.
    pub href: String,
    /// The link relationship type.
    pub rel: String,
    /// The language of the resource.
    pub hreflang: Option<String>,
    /// The MIME type of the resource.
    pub mime_type: Option<String>,
    /// Human-readable information about the link.
    pub title: Option<String>,
    /// The length of the resource, in bytes.
    pub length: Option<String>,
}

impl Default for Link {
    fn default() -> Self {
        Link {
            href: Default::default(),
            rel: "alternate".into(),
            hreflang: Default::default(),
            mime_type: Default::default(),
            title: Default::default(),
            length: Default::default(),
        }
    }
}

impl Link {
    /// Return the URI the referenced resource.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_href("http://example.com");
    /// assert_eq!(link.href(), "http://example.com");
    /// ```
    pub fn href(&self) -> &str {
        self.href.as_str()
    }

    /// Set the URI of the referenced resource.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_href("http://example.com");
    /// ```
    pub fn set_href<V>(&mut self, href: V)
    where
        V: Into<String>,
    {
        self.href = href.into()
    }

    /// Return the relation type of this link.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_rel("alternate");
    /// assert_eq!(link.rel(), "alternate");
    /// ```
    pub fn rel(&self) -> &str {
        self.rel.as_str()
    }

    /// Set the relation type of this link.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_rel("alternate");
    /// ```
    pub fn set_rel<V>(&mut self, rel: V)
    where
        V: Into<String>,
    {
        self.rel = rel.into()
    }

    /// Return the language of the referenced resource.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_hreflang("en".to_string());
    /// assert_eq!(link.hreflang(), Some("en"));
    /// ```
    pub fn hreflang(&self) -> Option<&str> {
        self.hreflang.as_deref()
    }

    /// Set the language of the referenced resource.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_hreflang("en".to_string());
    /// ```
    pub fn set_hreflang<V>(&mut self, hreflang: V)
    where
        V: Into<Option<String>>,
    {
        self.hreflang = hreflang.into()
    }

    /// Return the MIME type of the referenced resource.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_mime_type("text/html".to_string());
    /// assert_eq!(link.mime_type(), Some("text/html"));
    /// ```
    pub fn mime_type(&self) -> Option<&str> {
        self.mime_type.as_deref()
    }

    /// Set the MIME type of the referenced resource.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_mime_type("text/html".to_string());
    /// ```
    pub fn set_mime_type<V>(&mut self, mime_type: V)
    where
        V: Into<Option<String>>,
    {
        self.mime_type = mime_type.into()
    }

    /// Return the title of the referenced resource.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_title("Article Title".to_string());
    /// assert_eq!(link.title(), Some("Article Title"));
    /// ```
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Set the title of the referenced resource.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_title("Article Title".to_string());
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<Option<String>>,
    {
        self.title = title.into()
    }

    /// Return the content length of the referenced resource in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_length("1000".to_string());
    /// assert_eq!(link.length(), Some("1000"));
    /// ```
    pub fn length(&self) -> Option<&str> {
        self.length.as_deref()
    }

    /// Set the content length of the referenced resource in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Link;
    ///
    /// let mut link = Link::default();
    /// link.set_length("1000".to_string());
    /// ```
    pub fn set_length<V>(&mut self, length: V)
    where
        V: Into<Option<String>>,
    {
        self.length = length.into()
    }
}

impl Link {
    pub(crate) fn from_xml<'s, B: BufRead>(
        reader: &mut Reader<B>,
        element: &'s BytesStart<'s>,
    ) -> Result<Self, Error> {
        let mut link = Link::default();

        for att in element.attributes().with_checks(false).flatten() {
            match decode(att.key.as_ref(), reader)? {
                Cow::Borrowed("href") => link.href = attr_value(&att, reader)?.to_string(),
                Cow::Borrowed("rel") => link.rel = attr_value(&att, reader)?.to_string(),
                Cow::Borrowed("hreflang") => {
                    link.hreflang = Some(attr_value(&att, reader)?.to_string())
                }
                Cow::Borrowed("type") => {
                    link.mime_type = Some(attr_value(&att, reader)?.to_string())
                }
                Cow::Borrowed("title") => link.title = Some(attr_value(&att, reader)?.to_string()),
                Cow::Borrowed("length") => {
                    link.length = Some(attr_value(&att, reader)?.to_string())
                }
                _ => {}
            }
        }

        Ok(link)
    }
}

impl ToXml for Link {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let mut element = BytesStart::new("link");
        element.push_attribute(("href", &*self.href));
        element.push_attribute(("rel", &*self.rel));

        if let Some(ref hreflang) = self.hreflang {
            element.push_attribute(("hreflang", &**hreflang));
        }

        if let Some(ref mime_type) = self.mime_type {
            element.push_attribute(("type", &**mime_type));
        }

        if let Some(ref title) = self.title {
            element.push_attribute(("title", &**title));
        }

        if let Some(ref length) = self.length {
            element.push_attribute(("length", &**length));
        }

        writer
            .write_event(Event::Empty(element))
            .map_err(XmlError::new)?;

        Ok(())
    }
}

#[cfg(feature = "builders")]
impl LinkBuilder {
    /// Builds a new `Link`.
    pub fn build(&self) -> Link {
        self.build_impl().unwrap()
    }
}
