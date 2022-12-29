use std::io::Write;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;

use crate::error::XmlError;

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
