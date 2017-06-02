use category::Category;
use generator::Generator;
use link::Link;
use person::Person;

/// Represents a source in an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Source {
    /// A universally unique and permanent URI.
    id: String,
    /// A human-readable title for the feed.
    title: String,
    /// The last time the feed was modified in a significant way.
    updated: String,
    /// The authors of the feed.
    authors: Vec<Person>,
    /// The categories that the feed belongs to.
    categories: Vec<Category>,
    /// The contributors to the feed.
    contributors: Vec<Person>,
    /// The software used to generate the feed.
    generator: Option<Generator>,
    /// A small image which provides visual identification for the feed.
    icon: Option<String>,
    /// The Web pages related to the feed.
    links: Vec<Link>,
    /// A larger image which provides visual identification for the feed.
    logo: Option<String>,
    /// Information about rights held in and over the feed.
    rights: Option<String>,
    /// A human-readable description or subtitle for the feed.
    subtitle: Option<String>,
}
