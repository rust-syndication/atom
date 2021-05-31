use atom_syndication::{Feed, Text, TextType};
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
    let title = feed.title();
    assert_eq!(title, "&lt;p&gt;Feed Title&lt;/p&gt;");
    assert_eq!(title.base, None);
    assert_eq!(title.lang, None);
    assert_eq!(title.r#type, TextType::Html);
}

#[test]
fn text_cdata() {
    let reader = BufReader::new(File::open("tests/data/text_cdata.xml").unwrap());
    let feed = Feed::read_from(reader).unwrap();
    let title = feed.title();
    assert_eq!(title, "<p>Feed Title</p>");
    assert_eq!(title.base, None);
    assert_eq!(title.lang, None);
    assert_eq!(title.r#type, TextType::Html);
}

#[test]
fn text_empty() {
    let feed = feed!("tests/data/text_empty.xml");
    let title = feed.title();
    assert_eq!(title, "");
    assert_eq!(title.base, None);
    assert_eq!(title.lang, None);
    assert_eq!(title.r#type, TextType::Text);
}

#[test]
fn text_html() {
    let feed = feed!("tests/data/text_html.xml");
    let title = feed.title();
    assert_eq!(title, "<p>Feed Title</p>");
    assert_eq!(title.base, None);
    assert_eq!(title.lang, None);
    assert_eq!(title.r#type, TextType::Html);
}

#[test]
fn text_write_html() {
    let mut feed = Feed::default();
    feed.set_title(Text::html("<p>Feed Title</p>"));
    let xml = feed.to_string();
    assert!(xml.contains(r#"<title type="html">&lt;p&gt;Feed Title&lt;/p&gt;</title>"#));
}

#[test]
fn text_write_html_with_lang() {
    let mut feed = Feed::default();
    let mut title = Text::html("<p>Feed Title</p>");
    title.lang = Some("en".to_string());
    feed.set_title(title);
    let xml = feed.to_string();
    assert!(
        xml.contains(r#"<title xml:lang="en" type="html">&lt;p&gt;Feed Title&lt;/p&gt;</title>"#)
    );
}

#[test]
fn text_write_html_with_base_and_lang() {
    let mut feed = Feed::default();
    let mut title = Text::html("<p>Feed Title</p>");
    title.base = Some("http://example.com/articles/".to_string());
    title.lang = Some("en".to_string());
    feed.set_title(title);
    let xml = feed.to_string();
    assert!(xml.contains(r#"<title xml:base="http://example.com/articles/" xml:lang="en" type="html">&lt;p&gt;Feed Title&lt;/p&gt;</title>"#));
}

#[test]
fn text_write_xhtml() {
    let mut feed = Feed::default();
    feed.set_title(Text::xhtml(
        "<p xmlns=\"http://www.w3.org/1999/xhtml\">Feed Title</p>",
    ));
    let xml = feed.to_string();
    assert!(xml.contains(
        r#"<title type="xhtml"><p xmlns="http://www.w3.org/1999/xhtml">Feed Title</p></title>"#
    ));
}

#[test]
fn text_plain() {
    let feed = feed!("tests/data/text_plain.xml");
    let title = feed.title();
    assert_eq!(title, "Feed Title");
    assert_eq!(title.base, None);
    assert_eq!(title.lang, None);
    assert_eq!(title.r#type, TextType::Text);
}

#[test]
fn text_plain_escaped() {
    let feed = feed!("tests/data/text_plain_escaped.xml");
    let title = feed.title();
    assert_eq!(title, "<p>Feed Title</p>");
    assert_eq!(title.base, None);
    assert_eq!(title.lang, None);
    assert_eq!(title.r#type, TextType::Text);
}

#[test]
fn text_xhtml_escaped() {
    let feed = feed!("tests/data/text_xhtml_escaped.xml");
    let title = feed.title();
    assert_eq!(
        title,
        "<div xmlns=\"http://www.w3.org/1999/xhtml\">&lt;p&gt;Feed Title&lt;/p&gt;</div>"
    );
    assert_eq!(title.base, None);
    assert_eq!(title.lang, None);
    assert_eq!(title.r#type, TextType::Xhtml);
}

#[test]
fn text_xhtml() {
    let feed = feed!("tests/data/text_xhtml.xml");
    let title = feed.title();
    assert_eq!(
        title,
        "<div xmlns=\"http://www.w3.org/1999/xhtml\"><p>Feed Title</p></div>"
    );
    assert_eq!(title.base, None);
    assert_eq!(title.lang, None);
    assert_eq!(title.r#type, TextType::Xhtml);
}
