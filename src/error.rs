use std::error::Error as StdError;
use std::fmt;
use std::str::Utf8Error;

use quick_xml::errors::Error as XmlError;

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
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Xml(ref err) => err.description(),
            Error::Utf8(ref err) => err.description(),
            Error::InvalidStartTag => "input did not begin with an opening feed tag",
            Error::Eof => "unexpected end of input",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Xml(ref err) => Some(err),
            Error::Utf8(ref err) => Some(err),
            Error::InvalidStartTag | Error::Eof => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Xml(ref err) => err.fmt(f),
            Error::Utf8(ref err) => err.fmt(f),
            Error::InvalidStartTag => write!(f, "input did not begin with an opening feed tag"),
            Error::Eof => write!(f, "unexpected end of input"),
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
