/// Represents the generator of an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Generator {
    /// The name of the generator.
    value: String,
    /// The generator URI.
    uri: Option<String>,
    /// The generator version.
    version: Option<String>,
}
