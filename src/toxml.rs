use std::io::{self, Write};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;

use crate::error::XmlError;

/// Replaces the C0 control characters XML 1.0 forbids with U+FFFD. Dropping them
/// could join their neighbors into a new token.
pub(crate) struct XmlSafeWriter<W>(pub(crate) W);

impl<W: Write> Write for XmlSafeWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut runs = buf.split(|b| matches!(b, 0x00..=0x08 | 0x0B | 0x0C | 0x0E..=0x1F));
        if let Some(first) = runs.next() {
            self.0.write_all(first)?;
        }
        for run in runs {
            self.0.write_all("\u{FFFD}".as_bytes())?;
            self.0.write_all(run)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

pub(crate) trait ToXml {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError>;
}

impl<'a, T: ToXml> ToXml for &'a T {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        (*self).to_xml(writer)
    }
}

pub(crate) trait ToXmlNamed {
    fn to_xml_named<W>(&self, writer: &mut Writer<W>, name: &str) -> Result<(), XmlError>
    where
        W: Write;
}

impl<'a, T: ToXmlNamed> ToXmlNamed for &'a T {
    fn to_xml_named<W>(&self, writer: &mut Writer<W>, name: &str) -> Result<(), XmlError>
    where
        W: Write,
    {
        (*self).to_xml_named(writer, name)
    }
}

pub(crate) trait WriterExt {
    fn write_text_element(&mut self, name: &str, text: &str) -> Result<(), XmlError>;

    fn write_object<T>(&mut self, object: T) -> Result<(), XmlError>
    where
        T: ToXml;

    fn write_object_named<T>(&mut self, object: T, name: &str) -> Result<(), XmlError>
    where
        T: ToXmlNamed;

    fn write_objects<T, I>(&mut self, objects: I) -> Result<(), XmlError>
    where
        T: ToXml,
        I: IntoIterator<Item = T>;

    fn write_objects_named<T, I>(&mut self, objects: I, name: &str) -> Result<(), XmlError>
    where
        T: ToXmlNamed,
        I: IntoIterator<Item = T>;
}

impl<W: Write> WriterExt for Writer<W> {
    fn write_text_element(&mut self, name: &str, text: &str) -> Result<(), XmlError> {
        self.write_event(Event::Start(BytesStart::new(name)))
            .map_err(XmlError::new)?;
        self.write_event(Event::Text(BytesText::new(text)))
            .map_err(XmlError::new)?;
        self.write_event(Event::End(BytesEnd::new(name)))
            .map_err(XmlError::new)?;
        Ok(())
    }

    fn write_object<T>(&mut self, object: T) -> Result<(), XmlError>
    where
        T: ToXml,
    {
        object.to_xml(self)
    }

    fn write_object_named<T>(&mut self, object: T, name: &str) -> Result<(), XmlError>
    where
        T: ToXmlNamed,
    {
        object.to_xml_named(self, name)
    }

    fn write_objects<T, I>(&mut self, objects: I) -> Result<(), XmlError>
    where
        T: ToXml,
        I: IntoIterator<Item = T>,
    {
        for object in objects {
            object.to_xml(self)?;
        }

        Ok(())
    }

    fn write_objects_named<T, I>(&mut self, objects: I, name: &str) -> Result<(), XmlError>
    where
        T: ToXmlNamed,
        I: IntoIterator<Item = T>,
    {
        for object in objects {
            object.to_xml_named(self, name)?;
        }

        Ok(())
    }
}
