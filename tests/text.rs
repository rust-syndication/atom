use atom_syndication::{Feed, Text};
use std::fs::File;
use std::io::BufReader;

macro_rules! feed {
    ($f:expr) => {{
        let file = File::open($f).unwrap();
        let reader = BufReader::new(file);
        Feed::read_from(reader).unwrap()
    }};
}

#[test]
fn text_cdata_escaped() {
    let feed = feed!("tests/data/text_cdata_escaped.xml");
    assert_eq!(feed.title(), "&lt;p&gt;Feed Title&lt;/p&gt;");
}

#[test]
fn text_cdata() {
    let reader = BufReader::new(File::open("tests/data/text_cdata.xml").unwrap());
    let feed = Feed::read_from(reader).unwrap();
    assert_eq!(feed.title(), "<p>Feed Title</p>");
}

#[test]
fn text_empty() {
    let feed = feed!("tests/data/text_empty.xml");
    assert_eq!(feed.title(), "");
}

#[test]
fn text_html() {
    let feed = feed!("tests/data/text_html.xml");
    assert_eq!(feed.title(), "<p>Feed Title</p>");
}

#[test]
fn text_write_html() {
    let mut feed = Feed::default();
    feed.set_title(Text::html("<p>Feed Title</p>"));
    let xml = feed.to_string();
    assert!(xml.contains(r#"<title type="html">&lt;p&gt;Feed Title&lt;/p&gt;</title>"#));
}

#[test]
fn text_write_xhtml() {
    let mut feed = Feed::default();
    feed.set_title(Text::xhtml("<p>Feed Title</p>"));
    let xml = feed.to_string();
    assert!(xml.contains(r#"<title type="xhtml">&lt;p&gt;Feed Title&lt;/p&gt;</title>"#));
}

#[test]
fn text_plain() {
    let feed = feed!("tests/data/text_plain.xml");
    assert_eq!(feed.title(), "Feed Title");
}

#[test]
fn text_plain_escaped() {
    let feed = feed!("tests/data/text_plain_escaped.xml");
    assert_eq!(feed.title(), "<p>Feed Title</p>");
}

#[test]
fn text_xhtml_escaped() {
    let feed = feed!("tests/data/text_xhtml_escaped.xml");
    assert_eq!(
        feed.title(),
        "<div xmlns=\"http://www.w3.org/1999/xhtml\">&lt;p&gt;Feed Title&lt;/p&gt;</div>"
    );
}

#[test]
fn text_xhtml() {
    let feed = feed!("tests/data/text_xhtml.xml");
    assert_eq!(
        feed.title(),
        "<div xmlns=\"http://www.w3.org/1999/xhtml\"><p>Feed Title</p></div>"
    );
}
