extern crate atom_syndication as atom;

use std::io::BufReader;
use std::fs::File;

use atom::Feed;

macro_rules! feed {
    ($f:expr) => ({
        let file = File::open($f).unwrap();
        let reader = BufReader::new(file);
        Feed::read_from(reader).unwrap()
    })
}

#[test]
fn content_src() {
    let feed = feed!("tests/data/content_src.xml");
    let content = feed.entries().first().unwrap().content().unwrap();
    assert_eq!(content.value(), None);
    assert_eq!(content.src(), Some("http://example.com/image.png"));
    assert_eq!(content.content_type(), Some("image/png"));
}

#[test]
fn content_text_cdata_escaped() {
    let feed = feed!("tests/data/content_text_cdata_escaped.xml");
    let content = feed.entries().first().unwrap().content().unwrap();
    assert_eq!(content.value(), Some("&lt;p&gt;Entry content&lt;/p&gt;"));
    assert_eq!(content.content_type(), Some("text"));
}

#[test]
fn content_text_cdata() {
    let feed = feed!("tests/data/content_text_cdata.xml");
    let content = feed.entries().first().unwrap().content().unwrap();
    assert_eq!(content.value(), Some("<p>Entry content</p>"));
    assert_eq!(content.content_type(), Some("text"));
}

#[test]
fn content_text_html() {
    let feed = feed!("tests/data/content_text_html.xml");
    let content = feed.entries().first().unwrap().content().unwrap();
    assert_eq!(content.value(), Some("<p>Entry content</p>"));
    assert_eq!(content.content_type(), Some("html"));
}

#[test]
fn content_text_other() {
    let feed = feed!("tests/data/content_text_other.xml");
    let content = feed.entries().first().unwrap().content().unwrap();
    assert_eq!(content.value(), Some("RW50cnkgY29udGVudA=="));
    assert_eq!(content.content_type(), Some("application/octet-stream"));
}

#[test]
fn content_text_plain_escaped() {
    let feed = feed!("tests/data/content_text_plain_escaped.xml");
    let content = feed.entries().first().unwrap().content().unwrap();
    assert_eq!(content.value(), Some("<p>Entry content</p>"));
    assert_eq!(content.content_type(), Some("text"));
}

#[test]
fn content_text_plain() {
    let feed = feed!("tests/data/content_text_plain.xml");
    let content = feed.entries().first().unwrap().content().unwrap();
    assert_eq!(content.value(), Some("Entry content"));
    assert_eq!(content.content_type(), Some("text"));
}

#[test]
fn content_text_xhtml() {
    let feed = feed!("tests/data/content_text_xhtml.xml");
    let content = feed.entries().first().unwrap().content().unwrap();
    assert_eq!(
        content.value(),
        Some(
            "<div xmlns=\"http://www.w3.org/1999/xhtml\"><p>Entry content</p></div>",
        )
    );
    assert_eq!(content.content_type(), Some("xhtml"));
}
