extern crate atom_syndication as atom;

use std::fs::File;
use std::io::BufReader;

use crate::atom::{Category, Content, Entry, Feed, Text};

macro_rules! feed {
    ($f:expr) => {{
        let file = File::open($f).unwrap();
        let reader = BufReader::new(file);
        Feed::read_from(reader).unwrap()
    }};
}

#[test]
fn write_feed() {
    let feed = feed!("tests/data/feed.xml");
    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_entry() {
    let feed = feed!("tests/data/entry.xml");
    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_category() {
    let feed = feed!("tests/data/category.xml");
    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_generator() {
    let feed = feed!("tests/data/generator.xml");
    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_link() {
    let feed = feed!("tests/data/link.xml");
    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_person() {
    let feed = feed!("tests/data/person.xml");
    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_source() {
    let feed = feed!("tests/data/source.xml");
    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_extension() {
    let feed = feed!("tests/data/extension.xml");
    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_standalone_entry() {
    let file = File::open("tests/data/standalone_entry.xml").unwrap();
    let entry = Entry::read_from(BufReader::new(file)).unwrap();
    let out = entry.to_string();
    // A standalone entry document declares the Atom namespace on its root.
    assert!(
        out.starts_with("<?xml version=\"1.0\"?>\n<entry xmlns=\"http://www.w3.org/2005/Atom\" "),
        "out: {out}"
    );
    // The extension namespace declared on the parsed entry is re-emitted, so
    // the prefixed extension markup it contains stays namespace-valid.
    assert!(
        out.contains("xmlns:app=\"http://www.w3.org/2007/app\""),
        "out: {out}"
    );
    assert_eq!(out.parse::<Entry>().unwrap(), entry);
}

#[test]
fn write_embedded_entry_does_not_redeclare_xmlns() {
    let feed = feed!("tests/data/entry.xml");
    let out = feed.to_string();
    assert!(!out.contains("<entry xmlns"), "out: {out}");
}

#[test]
fn write_content_roundtrip() {
    let mut content = Content::default();
    content.set_base("http://example.com/blog/".to_string());
    content.set_content_type("html".to_string());
    content.set_value("<a href=\"2021-05-31/article.html\">Read more</a>".to_string());

    let mut entry = Entry::default();
    entry.set_content(content);

    let mut feed = Feed::default();
    feed.set_entries(vec![entry]);

    assert_eq!(feed.to_string().parse::<Feed>().unwrap(), feed);
}

#[test]
fn write_replaces_control_characters() {
    let mut category = Category::default();
    category.set_term("ta\u{8}g");

    let mut entry = Entry::default();
    entry.set_title("Ti\u{1e}tle\tand\nmore");
    entry.set_summary(Text::html("<scr\u{1}ipt>"));
    entry.set_categories(vec![category]);

    let mut feed = Feed::default();
    feed.set_entries(vec![entry]);

    let out = feed.to_string();
    // The parser is lenient, so check the raw output too.
    assert!(!out
        .bytes()
        .any(|b| b < 0x20 && !matches!(b, b'\t' | b'\n' | b'\r')));

    let parsed = out.parse::<Feed>().unwrap();
    let entry = &parsed.entries()[0];
    assert_eq!(entry.title().as_str(), "Ti\u{FFFD}tle\tand\nmore");
    assert_eq!(entry.summary().unwrap().as_str(), "<scr\u{FFFD}ipt>");
    assert_eq!(entry.categories()[0].term(), "ta\u{FFFD}g");
}
