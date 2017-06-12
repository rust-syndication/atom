extern crate atom_syndication as atom;

use std::io::BufReader;
use std::fs::File;

use atom::Feed;
use atom::extension::ExtensionMap;

macro_rules! feed {
    ($f:expr) => ({
        let file = File::open($f).unwrap();
        let reader = BufReader::new(file);
        Feed::read_from(reader).unwrap()
    })
}

#[test]
fn read_feed() {
    let feed = feed!("tests/data/feed.xml");
    assert_eq!(feed.title(), "Feed Title");
    assert_eq!(feed.id(), "urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    assert_eq!(feed.updated(), "2017-06-03T15:15:44-05:00");
    assert_eq!(feed.icon(), Some("http://example.com/icon.png"));
    assert_eq!(feed.logo(), Some("http://example.com/logo.png"));
    assert_eq!(feed.rights(), Some("© 2017 John Doe"));
    assert_eq!(feed.subtitle(), Some("Feed subtitle"));
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
    assert_eq!(entry.updated(), "2017-06-03T15:15:44-05:00");
    assert_eq!(entry.authors().len(), 2);
    assert_eq!(entry.categories().len(), 2);
    assert_eq!(entry.contributors().len(), 2);
    assert_eq!(entry.links().len(), 2);
    assert_eq!(entry.published(), Some("2017-06-01T15:15:44-05:00"));
    assert_eq!(entry.summary(), Some("Entry summary"));
    assert_eq!(entry.rights(), Some("© 2017 John Doe"));

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
    assert_eq!(source.updated(), "2017-06-03T15:15:44-05:00");
    assert_eq!(source.icon(), Some("http://example.com/icon.png"));
    assert_eq!(source.logo(), Some("http://example.com/logo.png"));
    assert_eq!(source.rights(), Some("© 2017 John Doe"));
    assert_eq!(source.subtitle(), Some("Feed subtitle"));
    assert_eq!(source.authors().len(), 2);
    assert_eq!(source.categories().len(), 2);
    assert_eq!(source.contributors().len(), 2);
    assert!(source.generator().is_some());
}

#[test]
fn read_extension() {
    let feed = feed!("tests/data/extension.xml");
    let entry = feed.entries().first().unwrap();

    assert_eq!(feed.namespaces().get("ext").map(|s| s.as_str()),
               Some("http://example.com"));

    let check_extensions = |extensions: &ExtensionMap| {
        assert!(extensions.contains_key("ext"));
        let map = extensions.get("ext").unwrap();

        assert!(map.contains_key("title"));
        let title = map.get("title").unwrap().first().unwrap();
        assert_eq!(title.value(), Some("Title"));
        assert_eq!(title.attrs().get("type").map(|s| s.as_str()), Some("text"));

        assert!(map.contains_key("parent"));
        let parent = map.get("parent").unwrap().first().unwrap();

        assert!(parent.children().contains_key("child"));
        let child = parent.children().get("child").unwrap().first().unwrap();
        assert_eq!(child.value(), Some("Child"));
    };


    check_extensions(feed.extensions());
    check_extensions(entry.extensions());
}
