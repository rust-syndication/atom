use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::Error;
use crate::fromxml::FromXml;
use crate::toxml::ToXml;

/// Represents a category in an Atom feed
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct Category {
    /// Identifies the category.
    term: String,
    /// Identifies the categorization scheme via a URI.
    scheme: Option<String>,
    /// A human-readable label for display.
    label: Option<String>,
}

impl Category {
    /// Return the term that identifies this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_term("technology");
    /// assert_eq!(category.term(), "technology");
    /// ```
    pub fn term(&self) -> &str {
        self.term.as_str()
    }

    /// Set the term that identifies this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_term("technology");
    /// ```
    pub fn set_term<V>(&mut self, term: V)
    where
        V: Into<String>,
    {
        self.term = term.into();
    }

    /// Return the categorization scheme URI.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_scheme("http://example.com/scheme".to_string());
    /// assert_eq!(category.scheme(), Some("http://example.com/scheme"));
    /// ```
    pub fn scheme(&self) -> Option<&str> {
        self.scheme.as_ref().map(String::as_str)
    }

    /// Set the categorization scheme URI.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_scheme("http://example.com/scheme".to_string());
    /// ```
    pub fn set_scheme<V>(&mut self, scheme: V)
    where
        V: Into<Option<String>>,
    {
        self.scheme = scheme.into();
    }

    /// Return the label for this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_scheme("Technology".to_string());
    /// assert_eq!(category.scheme(), Some("Technology"));
    /// ```

    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(String::as_str)
    }

    /// Set the label for this category.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Category;
    ///
    /// let mut category = Category::default();
    /// category.set_scheme("Technology".to_string());
    /// ```
    pub fn set_label<V>(&mut self, label: V)
    where
        V: Into<Option<String>>,
    {
        self.label = label.into();
    }
}

impl FromXml for Category {
    fn from_xml<B: BufRead>(reader: &mut Reader<B>, mut atts: Attributes) -> Result<Self, Error> {
        let mut category = Category::default();

        for attr in atts.with_checks(false) {
            if let Ok(att) = attr {
                match att.key {
                    b"term" => category.term = att.unescape_and_decode_value(reader)?,
                    b"scheme" => category.scheme = Some(att.unescape_and_decode_value(reader)?),
                    b"label" => category.label = Some(att.unescape_and_decode_value(reader)?),
                    _ => {}
                }
            }
        }

        reader.read_to_end(b"category", &mut Vec::new())?;

        Ok(category)
    }
}

impl ToXml for Category {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"category";
        let mut element = BytesStart::borrowed(name, name.len());
        element.push_attribute(("term", &*self.term));

        if let Some(ref scheme) = self.scheme {
            element.push_attribute(("scheme", &**scheme));
        }

        if let Some(ref label) = self.label {
            element.push_attribute(("label", &**label));
        }

        writer.write_event(Event::Empty(element))?;

        Ok(())
    }
}
