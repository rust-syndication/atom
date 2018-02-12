use std::io::{BufRead, Write};

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use error::Error;
use fromxml::FromXml;
use toxml::ToXml;
use util::atom_text;

/// Represents the generator of an Atom feed
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Default, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct Generator {
    /// The name of the generator.
    value: String,
    /// The generator URI.
    uri: Option<String>,
    /// The generator version.
    version: Option<String>,
}

impl Generator {
    /// Return the name of the generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Generator;
    ///
    /// let mut generator = Generator::default();
    /// generator.set_value("Feed Generator");
    /// assert_eq!(generator.value(), "Feed Generator");
    /// ```
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    /// Set the name of the generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Generator;
    ///
    /// let mut generator = Generator::default();
    /// generator.set_value("Feed Generator");
    /// assert_eq!(generator.value(), "Feed Generator");
    /// ```
    pub fn set_value<V>(&mut self, value: V)
    where
        V: Into<String>,
    {
        self.value = value.into()
    }

    /// Return the URI for the generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Generator;
    ///
    /// let mut generator = Generator::default();
    /// generator.set_uri("http://example.com/generator".to_string());
    /// assert_eq!(generator.uri(), Some("http://example.com/generator"));
    /// ```
    pub fn uri(&self) -> Option<&str> {
        self.uri.as_ref().map(|s| s.as_str())
    }

    /// Set the URI for the generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Generator;
    ///
    /// let mut generator = Generator::default();
    /// generator.set_uri("http://example.com/generator".to_string());
    /// ```
    pub fn set_uri<V>(&mut self, uri: V)
    where
        V: Into<Option<String>>,
    {
        self.uri = uri.into()
    }

    /// Return the version of the generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Generator;
    ///
    /// let mut generator = Generator::default();
    /// generator.set_version("1.0".to_string());
    /// assert_eq!(generator.version(), Some("1.0"));
    /// ```
    pub fn version(&self) -> Option<&str> {
        self.version.as_ref().map(|s| s.as_str())
    }

    /// Set the version of the generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Generator;
    ///
    /// let mut generator = Generator::default();
    /// generator.set_version("1.0".to_string());
    /// ```
    pub fn set_version<V>(&mut self, version: V)
    where
        V: Into<Option<String>>,
    {
        self.version = version.into()
    }
}

impl FromXml for Generator {
    fn from_xml<B: BufRead>(reader: &mut Reader<B>, mut atts: Attributes) -> Result<Self, Error> {
        let mut generator = Generator::default();

        for attr in atts.with_checks(false) {
            if let Ok(att) = attr {
                match att.key {
                    b"uri" => generator.uri = Some(att.unescape_and_decode_value(reader)?),
                    b"version" => generator.version = Some(att.unescape_and_decode_value(reader)?),
                    _ => {}
                }
            }
        }

        generator.value = atom_text(reader)?.unwrap_or_default();

        Ok(generator)
    }
}

impl ToXml for Generator {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"generator";
        let mut element = BytesStart::borrowed(name, name.len());

        if let Some(ref uri) = self.uri {
            element.push_attribute(("uri", &**uri));
        }

        if let Some(ref version) = self.version {
            element.push_attribute(("version", &**version));
        }

        writer.write_event(Event::Start(element))?;
        writer
            .write_event(Event::Text(BytesText::borrowed(self.value.as_bytes())))?;
        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;

        Ok(())
    }
}
