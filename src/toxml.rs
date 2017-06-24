use std::io::Write;

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::writer::Writer;

pub trait ToXml {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError>;
}

impl<'a, T: ToXml> ToXml for &'a T {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        (*self).to_xml(writer)
    }
}

pub trait ToXmlNamed {
    fn to_xml_named<W, N>(&self, writer: &mut Writer<W>, name: N) -> Result<(), XmlError>
    where
        W: Write,
        N: AsRef<[u8]>;
}

impl<'a, T: ToXmlNamed> ToXmlNamed for &'a T {
    fn to_xml_named<W, N>(&self, writer: &mut Writer<W>, name: N) -> Result<(), XmlError>
    where
        W: Write,
        N: AsRef<[u8]>,
    {
        (*self).to_xml_named(writer, name)
    }
}

pub trait WriterExt {
    fn write_text_element<N, T>(&mut self, name: N, text: T) -> Result<(), XmlError>
    where
        N: AsRef<[u8]>,
        T: AsRef<[u8]>;

    fn write_text_elements<N, T, I>(&mut self, name: N, values: I) -> Result<(), XmlError>
    where
        N: AsRef<[u8]>,
        T: AsRef<[u8]>,
        I: IntoIterator<Item = T>;

    fn write_object<T>(&mut self, object: T) -> Result<(), XmlError>
    where
        T: ToXml;

    fn write_object_named<T, N>(&mut self, object: T, name: N) -> Result<(), XmlError>
    where
        T: ToXmlNamed,
        N: AsRef<[u8]>;

    fn write_objects<T, I>(&mut self, objects: I) -> Result<(), XmlError>
    where
        T: ToXml,
        I: IntoIterator<Item = T>;

    fn write_objects_named<T, I, N>(&mut self, objects: I, name: N) -> Result<(), XmlError>
    where
        T: ToXmlNamed,
        I: IntoIterator<Item = T>,
        N: AsRef<[u8]>;
}

impl<W: Write> WriterExt for Writer<W> {
    fn write_text_element<N, T>(&mut self, name: N, text: T) -> Result<(), XmlError>
    where
        N: AsRef<[u8]>,
        T: AsRef<[u8]>,
    {
        let name = name.as_ref();
        self.write_event(Event::Start(BytesStart::borrowed(name, name.len())))?;
        self.write_event(Event::Text(BytesText::borrowed(text.as_ref())))?;
        self.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }

    fn write_text_elements<N, T, I>(&mut self, name: N, values: I) -> Result<(), XmlError>
    where
        N: AsRef<[u8]>,
        T: AsRef<[u8]>,
        I: IntoIterator<Item = T>,
    {
        for value in values {
            self.write_text_element(name.as_ref(), value)?;
        }

        Ok(())
    }

    fn write_object<T>(&mut self, object: T) -> Result<(), XmlError>
    where
        T: ToXml,
    {
        object.to_xml(self)
    }

    fn write_object_named<T, N>(&mut self, object: T, name: N) -> Result<(), XmlError>
    where
        T: ToXmlNamed,
        N: AsRef<[u8]>,
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

    fn write_objects_named<T, I, N>(&mut self, objects: I, name: N) -> Result<(), XmlError>
    where
        T: ToXmlNamed,
        I: IntoIterator<Item = T>,
        N: AsRef<[u8]>,
    {
        for object in objects {
            object.to_xml_named(self, name.as_ref())?;
        }

        Ok(())
    }
}
