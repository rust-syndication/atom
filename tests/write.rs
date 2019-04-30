extern crate atom_syndication as atom;

use std::fs::File;
use std::io::BufReader;

use crate::atom::Feed;

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
