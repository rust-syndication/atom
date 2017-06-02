/// Represents a category in an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Category {
    /// Identifies the category.
    term: String,
    /// Identifies the categorization scheme via a URI.
    scheme: Option<String>,
    /// A human-readable label for display.
    label: Option<String>,
}
