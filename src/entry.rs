use category::Category;
use content::Content;
use link::Link;
use source::Source;

/// Represents an entry in an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Entry {
    /// A universally unique and permanent URI.
    id: String,
    /// A human-readable title for the entry.
    title: String,
    /// The last time the entry was modified in a significant way.
    updated: String,
    /// The authors of the feed.
    authors: Vec<String>,
    /// The categories that the entry belongs to.
    categories: Vec<Category>,
    /// The contributors to the entry.
    contributors: Vec<Category>,
    /// The Web pages related to the entry.
    links: Vec<Link>,
    /// The time of the initial creation or first availability of the entry.
    published: Option<String>,
    /// The source information if an entry is copied from one feed into another feed.
    source: Option<Source>,
    /// A short summary, abstract, or excerpt of the entry.
    summary: Option<String>,
    /// Information about rights held in and over the entry.
    rights: Option<String>,
    /// Contains or links to the complete content of the entry.
    content: Option<Content>,
}
