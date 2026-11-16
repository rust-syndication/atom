use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::str::FromStr;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::category::Category;
use crate::content::Content;
use crate::error::{Error, XmlError};
use crate::extension::util::{extension_name, parse_extension};
use crate::extension::ExtensionMap;
use crate::feed::WriteConfig;
use crate::fromxml::FromXml;
use crate::link::Link;
use crate::person::Person;
use crate::source::Source;
use crate::text::Text;
use crate::toxml::{ToXml, WriterExt, XmlSafeWriter};
use crate::util::{
    atom_datetime, atom_text, attr_value, decode, default_fixed_datetime, skip, FixedDateTime,
};

/// Represents an entry in an Atom feed
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "std::convert::Infallible")
    )
)]
pub struct Entry {
    /// A human-readable title for the entry.
    pub title: Text,
    /// A universally unique and permanent URI.
    pub id: String,
    /// The last time the entry was modified.
    pub updated: FixedDateTime,
    /// The authors of the feed.
    #[cfg_attr(feature = "builders", builder(setter(each = "author")))]
    pub authors: Vec<Person>,
    /// The categories that the entry belongs to.
    #[cfg_attr(feature = "builders", builder(setter(each = "category")))]
    pub categories: Vec<Category>,
    /// The contributors to the entry.
    #[cfg_attr(feature = "builders", builder(setter(each = "contributor")))]
    pub contributors: Vec<Person>,
    /// The Web pages related to the entry.
    #[cfg_attr(feature = "builders", builder(setter(each = "link")))]
    pub links: Vec<Link>,
    /// The time of the initial creation or first availability of the entry.
    pub published: Option<FixedDateTime>,
    /// Information about rights held in and over the entry.
    pub rights: Option<Text>,
    /// The source information if an entry is copied from one feed into another feed.
    pub source: Option<Source>,
    /// A short summary, abstract, or excerpt of the entry.
    pub summary: Option<Text>,
    /// Contains or links to the complete content of the entry.
    pub content: Option<Content>,
    /// The extensions for this entry.
    #[cfg_attr(feature = "builders", builder(setter(each = "extension")))]
    pub extensions: ExtensionMap,
    /// The namespaces present in the entry tag.
    #[cfg_attr(feature = "builders", builder(setter(each = "namespace")))]
    pub namespaces: BTreeMap<String, String>,
}

impl Entry {
    /// Return the title of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_title("Entry Title");
    /// assert_eq!(entry.title(), "Entry Title");
    /// ```
    pub fn title(&self) -> &Text {
        &self.title
    }

    /// Set the title of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_title("Entry Title");
    /// ```
    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<Text>,
    {
        self.title = title.into();
    }

    /// Return the unique URI of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// assert_eq!(entry.id(), "urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// ```
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Set the unique URI of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// ```
    pub fn set_id<V>(&mut self, id: V)
    where
        V: Into<String>,
    {
        self.id = id.into();
    }

    /// Return the last time that this entry was modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    /// use atom_syndication::FixedDateTime;
    /// use std::str::FromStr;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_updated(FixedDateTime::from_str("2017-06-03T15:15:44-05:00").unwrap());
    /// assert_eq!(entry.updated().to_rfc3339(), "2017-06-03T15:15:44-05:00");
    /// ```
    pub fn updated(&self) -> &FixedDateTime {
        &self.updated
    }

    /// Set the last time that this entry was modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    /// use atom_syndication::FixedDateTime;
    /// use std::str::FromStr;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_updated(FixedDateTime::from_str("2017-06-03T15:15:44-05:00").unwrap());
    /// ```
    pub fn set_updated<V>(&mut self, updated: V)
    where
        V: Into<FixedDateTime>,
    {
        self.updated = updated.into();
    }

    /// Return the authors of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Person};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_authors(vec![Person::default()]);
    /// assert_eq!(entry.authors().len(), 1);
    /// ```
    pub fn authors(&self) -> &[Person] {
        self.authors.as_slice()
    }

    /// Set the authors of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Person};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_authors(vec![Person::default()]);
    /// ```
    pub fn set_authors<V>(&mut self, authors: V)
    where
        V: Into<Vec<Person>>,
    {
        self.authors = authors.into();
    }

    /// Return the categories this entry belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Category};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_categories(vec![Category::default()]);
    /// assert_eq!(entry.categories().len(), 1);
    /// ```
    pub fn categories(&self) -> &[Category] {
        self.categories.as_slice()
    }

    /// Set the categories this entry belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Category};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_categories(vec![Category::default()]);
    /// ```
    pub fn set_categories<V>(&mut self, categories: V)
    where
        V: Into<Vec<Category>>,
    {
        self.categories = categories.into();
    }

    /// Return the contributors to this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Person};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_contributors(vec![Person::default()]);
    /// assert_eq!(entry.contributors().len(), 1);
    /// ```
    pub fn contributors(&self) -> &[Person] {
        self.contributors.as_slice()
    }

    /// Set the contributors to this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Person};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_contributors(vec![Person::default()]);
    /// ```
    pub fn set_contributors<V>(&mut self, contributors: V)
    where
        V: Into<Vec<Person>>,
    {
        self.contributors = contributors.into();
    }

    /// Return the links for this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Link};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_links(vec![Link::default()]);
    /// assert_eq!(entry.links().len(), 1);
    /// ```
    pub fn links(&self) -> &[Link] {
        self.links.as_slice()
    }

    /// Set the links for this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Link};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_links(vec![Link::default()]);
    /// ```
    pub fn set_links<V>(&mut self, links: V)
    where
        V: Into<Vec<Link>>,
    {
        self.links = links.into();
    }

    /// Return the time that this entry was initially created or first made available.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    /// use atom_syndication::FixedDateTime;
    /// use std::str::FromStr;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_published(FixedDateTime::from_str("2017-06-01T15:15:44-05:00").unwrap());
    /// assert_eq!(entry.published().map(|x|x.to_rfc3339()), Some("2017-06-01T15:15:44-05:00".to_string()));
    /// ```
    pub fn published(&self) -> Option<&FixedDateTime> {
        self.published.as_ref()
    }

    /// Set the time that this entry was initially created or first made available.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    /// use atom_syndication::FixedDateTime;
    /// use std::str::FromStr;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_published(FixedDateTime::from_str("2017-06-01T15:15:44-05:00").unwrap());
    /// ```
    pub fn set_published<V>(&mut self, published: V)
    where
        V: Into<Option<FixedDateTime>>,
    {
        self.published = published.into();
    }

    /// Return the information about the rights held in and over this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Text};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_rights(Text::from("© 2017 John Doe"));
    /// assert_eq!(entry.rights().map(Text::as_str), Some("© 2017 John Doe"));
    /// ```
    pub fn rights(&self) -> Option<&Text> {
        self.rights.as_ref()
    }

    /// Set the information about the rights held in and over this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Text};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_rights(Text::from("© 2017 John Doe"));
    /// ```
    pub fn set_rights<V>(&mut self, rights: V)
    where
        V: Into<Option<Text>>,
    {
        self.rights = rights.into();
    }

    /// Return the source of this entry if it was copied from another feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Source};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_source(Source::default());
    /// assert!(entry.source().is_some());
    /// ```
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    /// Set the source of this entry if it was copied from another feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Source};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_source(Source::default());
    /// ```
    pub fn set_source<V>(&mut self, source: V)
    where
        V: Into<Option<Source>>,
    {
        self.source = source.into()
    }

    /// Return the summary of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Text};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_summary(Text::from("Entry summary."));
    /// assert_eq!(entry.summary().map(Text::as_str), Some("Entry summary."));
    /// ```
    pub fn summary(&self) -> Option<&Text> {
        self.summary.as_ref()
    }

    /// Set the summary of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Text};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_summary(Text::from("Entry summary."));
    /// ```
    pub fn set_summary<V>(&mut self, summary: V)
    where
        V: Into<Option<Text>>,
    {
        self.summary = summary.into();
    }

    /// Return the content of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Content};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_content(Content::default());
    /// assert!(entry.content().is_some());
    /// ```
    pub fn content(&self) -> Option<&Content> {
        self.content.as_ref()
    }

    /// Set the content of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Content};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_content(Content::default());
    /// assert!(entry.content().is_some());
    /// ```
    pub fn set_content<V>(&mut self, content: V)
    where
        V: Into<Option<Content>>,
    {
        self.content = content.into();
    }

    /// Return the extensions for this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use atom_syndication::Entry;
    /// use atom_syndication::extension::{ExtensionMap, Extension};
    ///
    /// let extension = Extension::default();
    ///
    /// let mut item_map = BTreeMap::<String, Vec<Extension>>::new();
    /// item_map.insert("ext:name".to_string(), vec![extension]);
    ///
    /// let mut extension_map = ExtensionMap::default();
    /// extension_map.insert("ext".to_string(), item_map);
    ///
    /// let mut entry = Entry::default();
    /// entry.set_extensions(extension_map);
    /// assert_eq!(entry.extensions()
    ///                 .get("ext")
    ///                 .and_then(|m| m.get("ext:name"))
    ///                 .map(|v| v.len()),
    ///            Some(1));
    /// ```
    pub fn extensions(&self) -> &ExtensionMap {
        &self.extensions
    }

    /// Set the extensions for this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    /// use atom_syndication::extension::ExtensionMap;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_extensions(ExtensionMap::default());
    /// ```
    pub fn set_extensions<V>(&mut self, extensions: V)
    where
        V: Into<ExtensionMap>,
    {
        self.extensions = extensions.into()
    }

    /// Return the namespaces for this entry.
    ///
    /// Combined with the extension map, this allows resolving the namespace
    /// URI an extension's prefix was bound to on the `<entry>` element.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// let mut namespaces = BTreeMap::new();
    /// namespaces.insert("ext".to_string(), "http://example.com".to_string());
    /// entry.set_namespaces(namespaces);
    /// assert_eq!(entry.namespaces().get("ext").map(|s| s.as_str()), Some("http://example.com"));
    /// ```
    pub fn namespaces(&self) -> &BTreeMap<String, String> {
        &self.namespaces
    }

    /// Set the namespaces for this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_namespaces(BTreeMap::new());
    /// ```
    pub fn set_namespaces<V>(&mut self, namespaces: V)
    where
        V: Into<BTreeMap<String, String>>,
    {
        self.namespaces = namespaces.into()
    }

    /// Attempt to read a standalone Atom entry from the reader.
    ///
    /// The prolog (XML declaration, processing instructions, comments) is
    /// skipped; the first start tag must be `<entry>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let xml = r#"<?xml version="1.0"?>
    /// <entry xmlns="http://www.w3.org/2005/Atom"><title>Entry Title</title></entry>"#;
    /// let entry = Entry::read_from(xml.as_bytes()).unwrap();
    /// assert_eq!(entry.title(), "Entry Title");
    /// ```
    pub fn read_from<B: BufRead>(reader: B) -> Result<Entry, Error> {
        let mut reader = Reader::from_reader(reader);
        reader.config_mut().expand_empty_elements = true;

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf).map_err(XmlError::new)? {
                Event::Start(element) => {
                    if decode(element.name().as_ref(), &reader)? == "entry" {
                        return Entry::from_xml(&mut reader, element.attributes());
                    } else {
                        return Err(Error::InvalidStartTag);
                    }
                }
                Event::Eof => break,
                _ => {}
            }

            buf.clear();
        }

        Err(Error::Eof)
    }

    /// Attempt to write this Atom entry as a standalone document to a writer
    /// using default `WriteConfig`.
    ///
    /// Unlike an entry embedded in a `<feed>`, a standalone `<entry>`
    /// document declares the Atom namespace on its root element.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let entry = Entry {
    ///     title: "Entry Title".into(),
    ///     id: "Entry ID".into(),
    ///     ..Default::default()
    /// };
    ///
    /// let out = entry.write_to(Vec::new())?;
    /// assert_eq!(&out, br#"<?xml version="1.0"?>
    /// <entry xmlns="http://www.w3.org/2005/Atom"><title>Entry Title</title><id>Entry ID</id><updated>1970-01-01T00:00:00+00:00</updated></entry>"#);
    /// # Ok(()) }
    /// ```
    pub fn write_to<W: Write>(&self, writer: W) -> Result<W, Error> {
        self.write_with_config(writer, WriteConfig::default())
    }

    /// Attempt to write this Atom entry as a standalone document to a writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, WriteConfig};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let entry = Entry {
    ///     title: "Entry Title".into(),
    ///     id: "Entry ID".into(),
    ///     ..Default::default()
    /// };
    ///
    /// let mut out = Vec::new();
    /// let config = WriteConfig {
    ///     write_document_declaration: false,
    ///     indent_size: Some(2),
    /// };
    /// entry.write_with_config(&mut out, config)?;
    /// assert_eq!(&out, br#"<entry xmlns="http://www.w3.org/2005/Atom">
    ///   <title>Entry Title</title>
    ///   <id>Entry ID</id>
    ///   <updated>1970-01-01T00:00:00+00:00</updated>
    /// </entry>"#);
    /// # Ok(()) }
    /// ```
    pub fn write_with_config<W: Write>(
        &self,
        writer: W,
        write_config: WriteConfig,
    ) -> Result<W, Error> {
        let mut writer = match write_config.indent_size {
            Some(indent_size) => Writer::new_with_indent(XmlSafeWriter(writer), b' ', indent_size),
            None => Writer::new(XmlSafeWriter(writer)),
        };
        if write_config.write_document_declaration {
            writer
                .write_event(Event::Decl(BytesDecl::new("1.0", None, None)))
                .map_err(XmlError::new)?;
            writer
                .write_event(Event::Text(BytesText::from_escaped("\n")))
                .map_err(XmlError::new)?;
        }
        self.to_xml_inner(&mut writer, true)?;
        Ok(writer.into_inner().0)
    }

    /// Serializes the `<entry>` element. `declare_xmlns` controls whether the
    /// root element declares the Atom namespace: standalone documents need it,
    /// entries embedded in a `<feed>` inherit it from the feed root.
    fn to_xml_inner<W: Write>(
        &self,
        writer: &mut Writer<W>,
        declare_xmlns: bool,
    ) -> Result<(), XmlError> {
        let name = "entry";
        let mut element = BytesStart::new(name);
        if declare_xmlns {
            element.push_attribute(("xmlns", "http://www.w3.org/2005/Atom"));
        }

        for (ns, uri) in &self.namespaces {
            element.push_attribute((format!("xmlns:{ns}").as_bytes(), uri.as_bytes()));
        }

        writer
            .write_event(Event::Start(element))
            .map_err(XmlError::new)?;
        writer.write_object_named(&self.title, "title")?;
        writer.write_text_element("id", &self.id)?;
        writer.write_text_element("updated", &self.updated.to_rfc3339())?;
        writer.write_objects_named(&self.authors, "author")?;
        writer.write_objects(&self.categories)?;
        writer.write_objects_named(&self.contributors, "contributor")?;
        writer.write_objects(&self.links)?;

        if let Some(ref published) = self.published {
            writer.write_text_element("published", &published.to_rfc3339())?;
        }

        if let Some(ref rights) = self.rights {
            writer.write_object_named(rights, "rights")?;
        }

        if let Some(ref source) = self.source {
            writer.write_object(source)?;
        }

        if let Some(ref summary) = self.summary {
            writer.write_object_named(summary, "summary")?;
        }

        if let Some(ref content) = self.content {
            writer.write_object(content)?;
        }

        for map in self.extensions.values() {
            for extensions in map.values() {
                writer.write_objects(extensions)?;
            }
        }

        writer
            .write_event(Event::End(BytesEnd::new(name)))
            .map_err(XmlError::new)?;

        Ok(())
    }
}

impl FromXml for Entry {
    fn from_xml<B: BufRead>(
        reader: &mut Reader<B>,
        mut atts: Attributes<'_>,
    ) -> Result<Self, Error> {
        let mut entry = Entry::default();
        let mut buf = Vec::new();

        for att in atts.with_checks(false).flatten() {
            match decode(att.key.as_ref(), reader)? {
                Cow::Borrowed("xmlns:dc") => {}
                key => {
                    if let Some(ns) = key.strip_prefix("xmlns:") {
                        entry
                            .namespaces
                            .insert(ns.to_string(), attr_value(&att, reader)?.to_string());
                    }
                }
            }
        }

        loop {
            match reader.read_event_into(&mut buf).map_err(XmlError::new)? {
                Event::Start(element) => match decode(element.name().as_ref(), reader)? {
                    Cow::Borrowed("id") => entry.id = atom_text(reader)?.unwrap_or_default(),
                    Cow::Borrowed("title") => {
                        entry.title = Text::from_xml(reader, element.attributes())?
                    }
                    Cow::Borrowed("updated") => {
                        entry.updated =
                            atom_datetime(reader)?.unwrap_or_else(default_fixed_datetime)
                    }
                    Cow::Borrowed("author") => entry
                        .authors
                        .push(Person::from_xml(reader, element.attributes())?),
                    Cow::Borrowed("category") => {
                        entry.categories.push(Category::from_xml(reader, &element)?);
                        skip(element.name(), reader)?;
                    }
                    Cow::Borrowed("contributor") => entry
                        .contributors
                        .push(Person::from_xml(reader, element.attributes())?),
                    Cow::Borrowed("link") => {
                        entry.links.push(Link::from_xml(reader, &element)?);
                        skip(element.name(), reader)?;
                    }
                    Cow::Borrowed("published") => entry.published = atom_datetime(reader)?,
                    Cow::Borrowed("rights") => {
                        entry.rights = Some(Text::from_xml(reader, element.attributes())?)
                    }
                    Cow::Borrowed("source") => {
                        entry.source = Some(Source::from_xml(reader, element.attributes())?)
                    }
                    Cow::Borrowed("summary") => {
                        entry.summary = Some(Text::from_xml(reader, element.attributes())?)
                    }
                    Cow::Borrowed("content") => {
                        entry.content = Some(Content::from_xml(reader, element.attributes())?)
                    }
                    n => {
                        if let Some((ns, name)) = extension_name(n.as_ref()) {
                            parse_extension(
                                reader,
                                element.attributes(),
                                ns,
                                name,
                                &mut entry.extensions,
                            )?;
                        } else {
                            skip(element.name(), reader)?;
                        }
                    }
                },
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        Ok(entry)
    }
}

impl ToXml for Entry {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        self.to_xml_inner(writer, false)
    }
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Entry::read_from(s.as_bytes())
    }
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        let buf = self.write_to(Vec::new()).unwrap_or_default();
        // this unwrap should be safe since the bytes written from the Entry are all valid utf8
        String::from_utf8(buf).unwrap()
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            title: Text::default(),
            id: String::new(),
            updated: default_fixed_datetime(),
            authors: Vec::new(),
            categories: Vec::new(),
            contributors: Vec::new(),
            links: Vec::new(),
            published: None,
            rights: None,
            source: None,
            summary: None,
            content: None,
            extensions: ExtensionMap::default(),
            namespaces: BTreeMap::default(),
        }
    }
}

#[cfg(feature = "builders")]
impl EntryBuilder {
    /// Builds a new `Entry`.
    pub fn build(&self) -> Entry {
        self.build_impl().unwrap()
    }
}
