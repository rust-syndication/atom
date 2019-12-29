use std::error::Error as StdError;
use std::fmt;
use std::str::Utf8Error;

use quick_xml::Error as XmlError;

#[derive(Debug)]
/// An error that occurred while performing an Atom operation.
pub enum Error {
    /// Unable to parse XML.
    Xml(XmlError),
    /// Unable to parse UTF8 in to a string.
    Utf8(Utf8Error),
    /// Input did not begin with an opening feed tag.
    InvalidStartTag,
    /// Unexpected end of input.
    Eof,
    /// The format of the timestamp is wrong.
    WrongDatetime(String),
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::Xml(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::InvalidStartTag => None,
            Error::Eof => None,
            Error::WrongDatetime(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Xml(ref err) => err.fmt(f),
            Error::Utf8(ref err) => err.fmt(f),
            Error::InvalidStartTag =>
                write!(f, "input did not begin with an opening feed tag"),
            Error::Eof =>
                write!(f, "unexpected end of input"),
            Error::WrongDatetime(ref datetime) =>
                write!(f, "timestamps must be formatted by RFC3339, rather than {}", datetime),
        }
    }
}

impl From<XmlError> for Error {
    fn from(err: XmlError) -> Error {
        Error::Xml(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}
