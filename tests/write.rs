extern crate atom_syndication as atom;

use std::fs::File;
use std::io::BufReader;

use crate::atom::{Content, Entry, Feed};

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
