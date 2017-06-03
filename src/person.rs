use std::io::BufRead;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use error::Error;
use fromxml::FromXml;
use util::atom_text;

/// Represents a person in an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Person {
    /// A human-readable name for the person.
    name: String,
    /// An email address for the person.
    email: Option<String>,
    /// A Web page for the person.
    uri: Option<String>,
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
        where V: Into<String>
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
        self.email.as_ref().map(|s| s.as_str())
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
        where V: Into<Option<String>>
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
        self.uri.as_ref().map(|s| s.as_str())
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
        where V: Into<Option<String>>
    {
        self.uri = uri.into()
    }
}

impl FromXml for Person {
    fn from_xml<B: BufRead>(reader: &mut Reader<B>, _: Attributes) -> Result<Self, Error> {
        let mut person = Person::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    match element.name() {
                        b"name" => person.name = atom_text(reader)?.unwrap_or_default(),
                        b"email" => person.email = atom_text(reader)?,
                        b"uri" => person.uri = atom_text(reader)?,
                        n => reader.read_to_end(n, &mut Vec::new())?,
                    }
                }
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        Ok(person)
    }
}
