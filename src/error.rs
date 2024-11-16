use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
/// An error that occurred while performing an Atom operation.
#[non_exhaustive]
pub enum Error {
    /// Unable to parse XML.
    Xml(XmlError),
    /// Input did not begin with an opening feed tag.
    InvalidStartTag,
    /// Unexpected end of input.
    Eof,
    /// The format of the timestamp is wrong.
    WrongDatetime(String),
    /// The value of an attribute is wrong.
    WrongAttribute {
        /// The name of the attribute.
        attribute: &'static str,
        /// Invalid value.
        value: String,
    },
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Xml(ref err) => Some(err),
            Error::InvalidStartTag => None,
            Error::Eof => None,
            Error::WrongDatetime(_) => None,
            Error::WrongAttribute { .. } => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Xml(ref err) => fmt::Display::fmt(err, f),
            Error::InvalidStartTag => write!(f, "input did not begin with an opening feed tag"),
            Error::Eof => write!(f, "unexpected end of input"),
            Error::WrongDatetime(ref datetime) => write!(
                f,
                "timestamps must be formatted by RFC3339, rather than {}",
                datetime
            ),
            Error::WrongAttribute {
                attribute,
                ref value,
            } => write!(
                f,
                "Unsupported value of attribute {}: '{}'.",
                attribute, value
            ),
        }
    }
}

impl From<XmlError> for Error {
    fn from(err: XmlError) -> Error {
        Error::Xml(err)
    }
}

#[derive(Debug)]
pub struct XmlError(Box<dyn StdError>);

impl XmlError {
    pub(crate) fn new(err: impl StdError + 'static) -> Self {
        Self(Box::new(err))
    }
}

impl StdError for XmlError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
