use std::str::Utf8Error;

use quick_xml::Error as XmlError;
use thiserror::Error;

#[derive(Debug, Error)]
/// An error that occurred while performing an Atom operation.
pub enum Error {
    /// Unable to parse XML.
    #[error("{0}")]
    Xml(#[from] XmlError),
    /// Unable to parse UTF8 in to a string.
    #[error("{0}")]
    Utf8(#[from] Utf8Error),
    /// Input did not begin with an opening feed tag.
    #[error("input did not begin with an opening feed tag")]
    InvalidStartTag,
    /// Unexpected end of input.
    #[error("unexpected end of input")]
    Eof,
    /// The format of the timestamp is wrong.
    #[error("timestamps must be formatted by RFC3339, rather than {0}")]
    WrongDatetime(String),
}
