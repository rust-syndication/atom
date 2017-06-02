/// Represents a person in an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Person {
    /// A human-readable name for the person.
    name: String,
    /// An email address for the person.
    email: Option<String>,
    /// A home page for the person.
    uri: Option<String>,
}
