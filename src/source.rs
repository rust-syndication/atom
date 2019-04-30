use std::io::{BufRead, Write};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;

use crate::category::Category;
use crate::error::Error;
use crate::fromxml::FromXml;
use crate::generator::Generator;
use crate::link::Link;
use crate::person::Person;
use crate::toxml::{ToXml, WriterExt};
use crate::util::{atom_datetime, atom_text, default_fixed_datetime, FixedDateTime};

/// Represents the source of an Atom entry
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(setter(into), default)]
pub struct Source {
    /// A human-readable title for the feed.
    title: String,
    /// A universally unique and permanent URI.
    id: String,
    /// The last time the feed was modified in a significant way.
    updated: FixedDateTime,
    /// The authors of the feed.
    authors: Vec<Person>,
    /// The categories that the feed belongs to.
    categories: Vec<Category>,
    /// The contributors to the feed.
    contributors: Vec<Person>,
    /// The software used to generate the feed.
    generator: Option<Generator>,
    /// A small image which provides visual identification for the feed.
    icon: Option<String>,
    /// The Web pages related to the feed.
    links: Vec<Link>,
    /// A larger image which provides visual identification for the feed.
    logo: Option<String>,
    /// Information about rights held in and over the feed.
    rights: Option<String>,
    /// A human-readable description or subtitle for the feed.
    subtitle: Option<String>,
}

impl Source {
    /// Return the title of the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_title("Feed Title");
    /// assert_eq!(source.title(), "Feed Title");
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Set the title of the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_title("Feed Title");
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<String>,
    {
        self.title = title.into();
    }

    /// Return the unique URI of the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// assert_eq!(source.id(), "urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// ```
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Set the unique URI of the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// ```
    pub fn set_id<V>(&mut self, id: V)
    where
        V: Into<String>,
    {
        self.id = id.into();
    }

    /// Return the last time that the source feed was modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    /// use atom_syndication::FixedDateTime;
    /// use std::str::FromStr;
    ///
    /// let mut source = Source::default();
    /// source.set_updated(FixedDateTime::from_str("2017-06-03T15:15:44-05:00").unwrap());
    /// assert_eq!(source.updated().to_rfc3339(), "2017-06-03T15:15:44-05:00");
    /// ```
    pub fn updated(&self) -> &FixedDateTime {
        &self.updated
    }

    /// Set the last time that the source feed was modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    /// use atom_syndication::FixedDateTime;
    /// use std::str::FromStr;
    ///
    /// let mut source = Source::default();
    /// source.set_updated(FixedDateTime::from_str("2017-06-03T15:15:44-05:00").unwrap());
    /// ```
    pub fn set_updated<V>(&mut self, updated: V)
    where
        V: Into<FixedDateTime>,
    {
        self.updated = updated.into();
    }

    /// Return the authors of the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Person};
    ///
    /// let mut source = Source::default();
    /// source.set_authors(vec![Person::default()]);
    /// assert_eq!(source.authors().len(), 1);
    /// ```
    pub fn authors(&self) -> &[Person] {
        self.authors.as_slice()
    }

    /// Set the authors of the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Person};
    ///
    /// let mut source = Source::default();
    /// source.set_authors(vec![Person::default()]);
    /// ```
    pub fn set_authors<V>(&mut self, authors: V)
    where
        V: Into<Vec<Person>>,
    {
        self.authors = authors.into();
    }

    /// Return the categories the source feed belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Category};
    ///
    /// let mut source = Source::default();
    /// source.set_categories(vec![Category::default()]);
    /// assert_eq!(source.categories().len(), 1);
    /// ```
    pub fn categories(&self) -> &[Category] {
        self.categories.as_slice()
    }

    /// Set the categories the source feed belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Category};
    ///
    /// let mut source = Source::default();
    /// source.set_categories(vec![Category::default()]);
    /// ```
    pub fn set_categories<V>(&mut self, categories: V)
    where
        V: Into<Vec<Category>>,
    {
        self.categories = categories.into();
    }

    /// Return the contributors to the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Person};
    ///
    /// let mut source = Source::default();
    /// source.set_contributors(vec![Person::default()]);
    /// assert_eq!(source.contributors().len(), 1);
    /// ```
    pub fn contributors(&self) -> &[Person] {
        self.contributors.as_slice()
    }

    /// Set the contributors to the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Person};
    ///
    /// let mut source = Source::default();
    /// source.set_contributors(vec![Person::default()]);
    /// ```
    pub fn set_contributors<V>(&mut self, contributors: V)
    where
        V: Into<Vec<Person>>,
    {
        self.contributors = contributors.into();
    }

    /// Return the name of the software used to generate the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Generator};
    ///
    /// let mut source = Source::default();
    /// source.set_generator(Generator::default());
    /// assert!(source.generator().is_some());
    /// ```
    pub fn generator(&self) -> Option<&Generator> {
        self.generator.as_ref()
    }

    /// Set the name of the software used to generate the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Generator};
    ///
    /// let mut source = Source::default();
    /// source.set_generator(Generator::default());
    /// ```
    pub fn set_generator<V>(&mut self, generator: V)
    where
        V: Into<Option<Generator>>,
    {
        self.generator = generator.into()
    }

    /// Return the icon for the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_icon("http://example.com/icon.png".to_string());
    /// assert_eq!(source.icon(), Some("http://example.com/icon.png"));
    /// ```
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_ref().map(String::as_str)
    }

    /// Set the icon for the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_icon("http://example.com/icon.png".to_string());
    /// ```
    pub fn set_icon<V>(&mut self, icon: V)
    where
        V: Into<Option<String>>,
    {
        self.icon = icon.into()
    }

    /// Return the Web pages related to the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Link};
    ///
    /// let mut source = Source::default();
    /// source.set_links(vec![Link::default()]);
    /// assert_eq!(source.links().len(), 1);
    /// ```
    pub fn links(&self) -> &[Link] {
        self.links.as_slice()
    }

    /// Set the Web pages related to the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Source, Link};
    ///
    /// let mut source = Source::default();
    /// source.set_links(vec![Link::default()]);
    /// ```
    pub fn set_links<V>(&mut self, links: V)
    where
        V: Into<Vec<Link>>,
    {
        self.links = links.into();
    }

    /// Return the logo for the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_logo("http://example.com/logo.png".to_string());
    /// assert_eq!(source.logo(), Some("http://example.com/logo.png"));
    /// ```
    pub fn logo(&self) -> Option<&str> {
        self.logo.as_ref().map(String::as_str)
    }

    /// Set the logo for the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_logo("http://example.com/logo.png".to_string());
    /// ```
    pub fn set_logo<V>(&mut self, logo: V)
    where
        V: Into<Option<String>>,
    {
        self.logo = logo.into()
    }

    /// Return the information about the rights held in and over the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_rights("© 2017 John Doe".to_string());
    /// assert_eq!(source.rights(), Some("© 2017 John Doe"));
    /// ```
    pub fn rights(&self) -> Option<&str> {
        self.rights.as_ref().map(String::as_str)
    }

    /// Set the information about the rights held in and over the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_rights("© 2017 John Doe".to_string());
    /// ```
    pub fn set_rights<V>(&mut self, rights: V)
    where
        V: Into<Option<String>>,
    {
        self.rights = rights.into()
    }

    /// Return the description or subtitle of the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_subtitle("Feed subtitle".to_string());
    /// assert_eq!(source.subtitle(), Some("Feed subtitle"));
    /// ```
    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_ref().map(String::as_str)
    }

    /// Set the description or subtitle of the source feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Source;
    ///
    /// let mut source = Source::default();
    /// source.set_subtitle("Feed subtitle".to_string());
    /// ```
    pub fn set_subtitle<V>(&mut self, subtitle: V)
    where
        V: Into<Option<String>>,
    {
        self.subtitle = subtitle.into()
    }
}

impl FromXml for Source {
    fn from_xml<B: BufRead>(reader: &mut Reader<B>, _: Attributes) -> Result<Self, Error> {
        let mut source = Source::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => match element.name() {
                    b"id" => source.id = atom_text(reader)?.unwrap_or_default(),
                    b"title" => source.title = atom_text(reader)?.unwrap_or_default(),
                    b"updated" => {
                        source.updated =
                            atom_datetime(reader)?.unwrap_or_else(default_fixed_datetime)
                    }
                    b"author" => source
                        .authors
                        .push(Person::from_xml(reader, element.attributes())?),
                    b"category" => source
                        .categories
                        .push(Category::from_xml(reader, element.attributes())?),
                    b"contributor" => source
                        .contributors
                        .push(Person::from_xml(reader, element.attributes())?),
                    b"generator" => {
                        source.generator = Some(Generator::from_xml(reader, element.attributes())?)
                    }
                    b"icon" => source.icon = atom_text(reader)?,
                    b"link" => source
                        .links
                        .push(Link::from_xml(reader, element.attributes())?),
                    b"logo" => source.logo = atom_text(reader)?,
                    b"rights" => source.rights = atom_text(reader)?,
                    b"subtitle" => source.subtitle = atom_text(reader)?,
                    n => reader.read_to_end(n, &mut Vec::new())?,
                },
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        Ok(source)
    }
}

impl ToXml for Source {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = b"source";
        writer.write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;
        writer.write_text_element(b"title", &*self.title)?;
        writer.write_text_element(b"id", &*self.id)?;
        writer.write_text_element(b"updated", &self.updated.to_rfc3339())?;
        writer.write_objects_named(&self.authors, "author")?;
        writer.write_objects(&self.categories)?;
        writer.write_objects_named(&self.contributors, "contributor")?;

        if let Some(ref generator) = self.generator {
            writer.write_object(generator)?;
        }

        if let Some(ref icon) = self.icon {
            writer.write_text_element(b"icon", &**icon)?;
        }

        writer.write_objects(&self.links)?;

        if let Some(ref logo) = self.logo {
            writer.write_text_element(b"logo", &**logo)?;
        }

        if let Some(ref rights) = self.rights {
            writer.write_text_element(b"rights", &**rights)?;
        }

        if let Some(ref subtitle) = self.subtitle {
            writer.write_text_element(b"subtitle", &**subtitle)?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;

        Ok(())
    }
}

impl Default for Source {
    fn default() -> Self {
        Source {
            title: String::new(),
            id: String::new(),
            updated: default_fixed_datetime(),
            authors: Vec::new(),
            categories: Vec::new(),
            contributors: Vec::new(),
            generator: None,
            icon: None,
            links: Vec::new(),
            logo: None,
            rights: None,
            subtitle: None,
        }
    }
}
