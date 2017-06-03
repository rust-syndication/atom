#![warn(missing_docs)]

#![doc(html_root_url = "https://docs.rs/rss/")]

//! Library for serializing the Atom web content syndication format.

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
mod util;

pub use feed::Feed;
pub use category::Category;
pub use content::Content;
pub use entry::Entry;
pub use generator::Generator;
pub use link::Link;
pub use person::Person;
pub use source::Source;
pub use error::Error;
