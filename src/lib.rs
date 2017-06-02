#![doc(html_root_url = "https://docs.rs/rss/")]

//! Library for serializing the Atom web content syndication format.

extern crate quick_xml;

mod feed;
pub use feed::Feed;

mod category;
pub use category::Category;

mod content;
pub use content::Content;

mod entry;
pub use entry::Entry;

mod generator;
pub use generator::Generator;

mod link;
pub use link::Link;

mod person;
pub use person::Person;

mod source;
pub use source::Source;
