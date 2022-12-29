use std::borrow::Cow;
use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::error::{Error, XmlError};
use crate::fromxml::FromXml;
use crate::toxml::{ToXmlNamed, WriterExt};
use crate::util::{atom_text, decode, skip};

/// Represents a person in an Atom feed
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
pub struct Person {
    /// A human-readable name for the person.
    pub name: String,
    /// An email address for the person.
    pub email: Option<String>,
    /// A Web page for the person.
    pub uri: Option<String>,
}

impl Person {
    /// Return the name of this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Person;
    ///
    /// let mut person = Person::default();
    /// person.set_name("John Doe");
    /// assert_eq!(person.name(), "John Doe");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the name of this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Person;
    ///
    /// let mut person = Person::default();
    /// person.set_name("John Doe");
    /// ```
    pub fn set_name<V>(&mut self, name: V)
    where
        V: Into<String>,
    {
        self.name = name.into()
    }

    /// Return the email address for this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Person;
    ///
    /// let mut person = Person::default();
    /// person.set_email("johndoe@example.com".to_string());
    /// assert_eq!(person.email(), Some("johndoe@example.com"));
    /// ```
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// Set the email address for this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Person;
    ///
    /// let mut person = Person::default();
    /// person.set_email("johndoe@example.com".to_string());
    /// ```
    pub fn set_email<V>(&mut self, email: V)
    where
        V: Into<Option<String>>,
    {
        self.email = email.into()
    }

    /// Return the Web page for this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Person;
    ///
    /// let mut person = Person::default();
    /// person.set_uri("http://example.com".to_string());
    /// assert_eq!(person.uri(), Some("http://example.com"));
    /// ```
    pub fn uri(&self) -> Option<&str> {
        self.uri.as_deref()
    }

    /// Set the Web page for this person.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Person;
    ///
    /// let mut person = Person::default();
    /// person.set_uri("http://example.com".to_string());
    /// ```
    pub fn set_uri<V>(&mut self, uri: V)
    where
        V: Into<Option<String>>,
    {
        self.uri = uri.into()
    }
}

impl FromXml for Person {
    fn from_xml<B: BufRead>(reader: &mut Reader<B>, _: Attributes<'_>) -> Result<Self, Error> {
        let mut person = Person::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf).map_err(XmlError::new)? {
                Event::Start(element) => match decode(element.name().as_ref(), reader)? {
                    Cow::Borrowed("name") => person.name = atom_text(reader)?.unwrap_or_default(),
                    Cow::Borrowed("email") => person.email = atom_text(reader)?,
                    Cow::Borrowed("uri") => person.uri = atom_text(reader)?,
                    _ => skip(element.name(), reader)?,
                },
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        Ok(person)
    }
}

impl ToXmlNamed for Person {
    fn to_xml_named<W>(&self, writer: &mut Writer<W>, name: &str) -> Result<(), XmlError>
    where
        W: Write,
    {
        writer
            .write_event(Event::Start(BytesStart::new(name)))
            .map_err(XmlError::new)?;
        writer.write_text_element("name", &self.name)?;

        if let Some(ref email) = self.email {
            writer.write_text_element("email", email)?;
        }

        if let Some(ref uri) = self.uri {
            writer.write_text_element("uri", uri)?;
        }

        writer
            .write_event(Event::End(BytesEnd::new(name)))
            .map_err(XmlError::new)?;

        Ok(())
    }
}

#[cfg(feature = "builders")]
impl PersonBuilder {
    /// Builds a new `Person`.
    pub fn build(&self) -> Person {
        self.build_impl().unwrap()
    }
}
