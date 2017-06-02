#[derive(Debug, Default, Clone, PartialEq)]
pub struct Content {
    /// The text value of the content.
    value: String,
    /// The URI of where the content can be found.
    src: Option<String>,
    /// The media type of the content.
    content_type: Option<String>,
}
