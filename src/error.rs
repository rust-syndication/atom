use std::str::Utf8Error;

use quick_xml::Error as XmlError;

#[derive(Debug, Fail)]
/// An error that occurred while performing an Atom operation.
pub enum Error {
    /// Unable to parse XML.
    #[fail(display = "{}", _0)]
    Xml(#[cause] XmlError),
    /// Unable to parse UTF8 in to a string.
    #[fail(display = "{}", _0)]
    Utf8(#[cause] Utf8Error),
    /// Input did not begin with an opening feed tag.
    #[fail(display = "input did not begin with an opening feed tag")]
    InvalidStartTag,
    /// Unexpected end of input.
    #[fail(display = "unexpected end of input")]
    Eof,
    /// The format of the timestamp is wrong.
    #[fail(
        display = "timestamps must be formatted by RFC3339, rather than {}",
        _0
    )]
    WrongDatetime(String),
    /// An error during the web request.
    #[cfg(feature = "from_url")]
    #[fail(display = "{}", _0)]
    UrlRequest(#[cause] ::reqwest::Error),
    /// An IO error.
    #[cfg(feature = "from_url")]
    #[fail(display = "{}", _0)]
    Io(#[cause] ::std::io::Error),
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

#[cfg(feature = "from_url")]
impl From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Error {
        Error::UrlRequest(err)
    }
}

#[cfg(feature = "from_url")]
impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}
