extern crate atom_syndication as atom;

use std::fs::File;
use std::io::BufReader;

use atom::Error;

use crate::atom::extension::ExtensionMap;
use crate::atom::{Feed, Text};

macro_rules! feed {
    ($f:expr) => {{
        let file = File::open($f).unwrap();
        let reader = BufReader::new(file);
        Feed::read_from(reader).unwrap()
    }};
}

#[test]
fn read_feed() {
    let feed = feed!("tests/data/feed.xml");
    assert_eq!(feed.title(), "Feed Title");
    assert_eq!(feed.id(), "urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    assert_eq!(feed.updated().to_rfc3339(), "2017-06-03T15:15:44-05:00");
    assert_eq!(feed.icon(), Some("http://example.com/icon.png"));
    assert_eq!(feed.logo(), Some("http://example.com/logo.png"));
    assert_eq!(feed.rights().map(Text::as_str), Some("© 2017 John Doe"));
    assert_eq!(feed.subtitle().map(Text::as_str), Some("Feed subtitle"));
    assert_eq!(feed.authors().len(), 2);
    assert_eq!(feed.categories().len(), 2);
    assert_eq!(feed.contributors().len(), 2);
    assert!(feed.generator().is_some());
    assert_eq!(feed.links().len(), 2);
}

#[test]
fn read_entry() {
    let feed = feed!("tests/data/entry.xml");
    assert_eq!(feed.entries().len(), 1);

    let entry = feed.entries().first().unwrap();
    assert_eq!(entry.title(), "Entry Title");
    assert_eq!(entry.id(), "http://example.com/article/1");
    assert_eq!(entry.updated().to_rfc3339(), "2017-06-03T15:15:44-05:00");
    assert_eq!(entry.authors().len(), 2);
    assert_eq!(entry.categories().len(), 2);
    assert_eq!(entry.contributors().len(), 2);
    assert_eq!(entry.links().len(), 2);
    assert_eq!(
        entry.published().map(chrono::DateTime::to_rfc3339),
        Some("2017-06-01T15:15:44-05:00".to_string())
    );
    assert_eq!(entry.summary().map(Text::as_str), Some("Entry summary"));
    assert_eq!(entry.rights().map(Text::as_str), Some("© 2017 John Doe"));

    let content = entry.content().unwrap();
    assert_eq!(content.value(), Some("Entry content"));
}

#[test]
fn read_entry_with_non_standard_dates() {
    let feed = feed!("tests/data/entry_with_non_standard_dates.xml");
    assert_eq!(feed.entries().len(), 1);

    let entry = feed.entries().first().unwrap();
    assert_eq!(entry.title(), "Entry Title");
    assert_eq!(entry.id(), "http://example.com/article/1");
    assert_eq!(entry.updated().to_rfc3339(), "2017-06-03T15:15:44-05:00");
    assert_eq!(entry.authors().len(), 2);
    assert_eq!(entry.categories().len(), 2);
    assert_eq!(entry.contributors().len(), 2);
    assert_eq!(entry.links().len(), 2);
    assert_eq!(
        entry.published().map(chrono::DateTime::to_rfc3339),
        Some("2017-06-01T15:15:44-05:00".to_string())
    );
    assert_eq!(entry.summary().map(Text::as_str), Some("Entry summary"));
    assert_eq!(entry.rights().map(Text::as_str), Some("© 2017 John Doe"));

    let content = entry.content().unwrap();
    assert_eq!(content.value(), Some("Entry content"));
}

#[test]
fn read_category() {
    let feed = feed!("tests/data/category.xml");
    let category = feed.categories().first().unwrap();
    assert_eq!(category.term(), "technology");
    assert_eq!(category.scheme(), Some("http://example.com/scheme"));
    assert_eq!(category.label(), Some("Technology"));
}

#[test]
fn read_generator() {
    let feed = feed!("tests/data/generator.xml");
    let generator = feed.generator().unwrap();
    assert_eq!(generator.value(), "Example Generator");
    assert_eq!(generator.uri(), Some("http://example.com/generator"));
    assert_eq!(generator.version(), Some("1.0"));
}

#[test]
fn read_link() {
    let feed = feed!("tests/data/link.xml");
    let link = feed.links().first().unwrap();
    assert_eq!(link.rel(), "enclosure");
    assert_eq!(link.href(), "http://example.com/audio.mp3");
    assert_eq!(link.hreflang(), Some("en"));
    assert_eq!(link.mime_type(), Some("audio/mpeg"));
    assert_eq!(link.title(), Some("audio"));
    assert_eq!(link.length(), Some("1000"));
}

#[test]
fn read_person() {
    let feed = feed!("tests/data/person.xml");
    let person = feed.authors().first().unwrap();
    assert_eq!(person.name(), "John Doe");
    assert_eq!(person.email(), Some("johndoe@example.com"));
    assert_eq!(person.uri(), Some("http://example.com"));
}

#[test]
fn read_source() {
    let feed = feed!("tests/data/source.xml");

    let entry = feed.entries().first().unwrap();
    assert!(entry.source().is_some());

    let source = entry.source().unwrap();
    assert_eq!(source.title(), "Feed Title");
    assert_eq!(source.id(), "urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    assert_eq!(source.updated().to_rfc3339(), "2017-06-03T15:15:44-05:00");
    assert_eq!(source.icon(), Some("http://example.com/icon.png"));
    assert_eq!(source.logo(), Some("http://example.com/logo.png"));
    assert_eq!(source.rights().map(Text::as_str), Some("© 2017 John Doe"));
    assert_eq!(source.subtitle().map(Text::as_str), Some("Feed subtitle"));
    assert_eq!(source.authors().len(), 2);
    assert_eq!(source.categories().len(), 2);
    assert_eq!(source.contributors().len(), 2);
    assert!(source.generator().is_some());
}

#[test]
fn read_extension() {
    let feed = feed!("tests/data/extension.xml");
    let entry = feed.entries().first().unwrap();

    assert_eq!(
        feed.namespaces().get("ext").map(String::as_str),
        Some("http://example.com")
    );

    let check_extensions = |extensions: &ExtensionMap| {
        assert!(extensions.contains_key("ext"));
        let map = extensions.get("ext").unwrap();

        assert!(map.contains_key("title"));
        let title = map.get("title").unwrap().first().unwrap();
        assert_eq!(title.value(), Some("<strong>Title</strong>"));
        assert_eq!(title.attrs().get("type").map(String::as_str), Some("text"));

        assert!(map.contains_key("parent"));
        let parent = map.get("parent").unwrap().first().unwrap();

        assert!(parent.children().contains_key("child"));
        let child = parent.children().get("child").unwrap().first().unwrap();
        assert_eq!(child.value(), Some("Child"));
    };

    check_extensions(feed.extensions());
    check_extensions(entry.extensions());
}

#[test]
fn read_eof() {
    let result = Feed::read_from("".as_bytes());
    assert!(matches!(result, Err(Error::Eof)));
}

#[test]
fn read_invalid_start() {
    let result = Feed::read_from("<wrong></wrong>".as_bytes());
    assert!(matches!(result, Err(Error::InvalidStartTag)));
}

#[test]
fn read_invalid_attribute_lang() {
    let result = Feed::read_from("<feed xml:lang=\"&;\"></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn read_invalid_attribute_base() {
    let result = Feed::read_from("<feed xml:base=\"&;\"></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn read_invalid_attribute_namespace() {
    let result = Feed::read_from("<feed xmlns:invalid=\"&;\"></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn read_mismatched_tags() {
    let result = Feed::read_from("<feed><a></b></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn read_internal_invalid_tag() {
    let result = Feed::read_from("<feed><aba><aaa></aba></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn read_entry_internal_invalid_tag() {
    let result = Feed::read_from("<feed><entry><aaa></entry></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn link_invalid_attribute() {
    let result = Feed::read_from("<feed><link href=\"&;\"></link></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn text_invalid_xml_base() {
    let result = Feed::read_from("<feed><rights xml:base=\"&;\"></rights></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn text_invalid_xml_lang() {
    let result = Feed::read_from("<feed><rights xml:lang=\"&;\"></rights></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn text_invalid_type() {
    let result = Feed::read_from("<feed><rights type=\"&;\"></rights></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn author_internal_invalid_tag() {
    let result = Feed::read_from("<feed><author><invalid></author></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn source_internal_invalid_tag() {
    let result =
        Feed::read_from("<feed><entry><source><invalid></source></entry></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn content_invalid_xml_lang() {
    let result = Feed::read_from(
        "<feed><entry><content xml:lang=\"&;\"></content></entry></feed>".as_bytes(),
    );
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn content_invalid_xml_base() {
    let result = Feed::read_from(
        "<feed><entry><content xml:base=\"&;\"></content></entry></feed>".as_bytes(),
    );
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn content_invalid_type() {
    let result =
        Feed::read_from("<feed><entry><content type=\"&;\"></content></entry></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn content_invalid_src() {
    let result =
        Feed::read_from("<feed><entry><content src=\"&;\"></content></entry></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn category_invalid_term() {
    let result = Feed::read_from("<feed><category term=\"&;\"></category></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn category_invalid_scheme() {
    let result = Feed::read_from("<feed><category scheme=\"&;\"></category></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn category_invalid_label() {
    let result = Feed::read_from("<feed><category label=\"&;\"></category></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn generator_invalid_uri() {
    let result = Feed::read_from("<feed><generator uri=\"&;\"></generator></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}

#[test]
fn generator_invalid_version() {
    let result = Feed::read_from("<feed><generator version=\"&;\"></generator></feed>".as_bytes());
    assert!(matches!(result, Err(Error::Xml(_))));
}
