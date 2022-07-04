use std::io::BufRead;

use quick_xml::events::attributes::Attributes;
use quick_xml::Reader;

use crate::error::Error;

pub(crate) trait FromXml: Sized {
    fn from_xml<R: BufRead>(reader: &mut Reader<R>, atts: Attributes<'_>) -> Result<Self, Error>;
}
