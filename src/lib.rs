#![warn(missing_docs)]

#![doc(html_root_url = "https://docs.rs/atom_syndication/")]

//! Library for serializing the Atom web content syndication format.
//!
//! # Reading
//!
//! A feed can be read from any object that implements the `BufRead` trait or using
//! the `FromStr` trait.
//!
//! ```no_run
//! use std::fs::File;
//! use std::io::BufReader;
//! use atom_syndication::Feed;
//!
//! let file = File::open("example.xml").unwrap();
//! let feed = Feed::read_from(BufReader::new(file)).unwrap();
//!
//! let string = "<feed></feed>";
//! let feed = string.parse::<Feed>().unwrap();
//! ```
//!
//! # Writing
//!
//! A feed can be written to any object that implements the `Write` trait or converted to an XML
//! string using the `ToString` trait.
//!
//! **Note**: Writing a feed does not perform any escaping of XML entities.
//!
//! ## Example
//!
//! ```no_run
//! use std::fs::File;
//! use std::io::{BufReader, sink};
//! use atom_syndication::Feed;
//!
//! let file = File::open("example.xml").unwrap();
//! let feed = Feed::read_from(BufReader::new(file)).unwrap();
//!
//! // write to the feed to a writer
//! feed.write_to(sink()).unwrap();
//!
//! // convert the feed to a string
//! let string = feed.to_string();
//! ```

extern crate quick_xml;

mod feed;
mod category;
mod content;
mod entry;
mod generator;
mod link;
mod person;
mod source;

mod error;
mod fromxml;
mod toxml;
mod util;

/// Types and functions for namespaced extensions.
pub mod extension;

pub use feed::Feed;
pub use category::Category;
pub use content::Content;
pub use entry::Entry;
pub use generator::Generator;
pub use link::Link;
pub use person::Person;
pub use source::Source;
pub use error::Error;
