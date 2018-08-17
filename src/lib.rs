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

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate failure;

extern crate quick_xml;

extern crate chrono;

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

pub use feed::{Feed, FeedBuilder};
pub use category::{Category, CategoryBuilder};
pub use content::{Content, ContentBuilder};
pub use entry::{Entry, EntryBuilder};
pub use generator::{Generator, GeneratorBuilder};
pub use link::{Link, LinkBuilder};
pub use person::{Person, PersonBuilder};
pub use source::{Source, SourceBuilder};
pub use error::Error;
pub use util::FixedDateTime;