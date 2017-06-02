/// Represents a link in an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Link {
    /// The URI of the referenced resource.
    href: String,
    /// The language of the resource.
    hreflang: Option<String>,
    /// The link relationship type.
    rel: Option<String>,
    /// The MIME type of the resource.
    mime_type: Option<String>,
    /// Human readable information about the link.
    title: Option<String>,
    /// The length of the resource, in bytes.
    length: Option<i64>,
}
