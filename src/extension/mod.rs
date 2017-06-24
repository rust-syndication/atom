use std::collections::HashMap;
use std::io::Write;
use std::str;

use quick_xml::errors::Error as XmlError;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::writer::Writer;

use toxml::ToXml;

pub(crate) mod util;

/// A map of extension namespace prefixes to local names to elements.
pub type ExtensionMap = HashMap<String, HashMap<String, Vec<Extension>>>;

/// A namespaced extension.
#[derive(Debug, Default, Clone, PartialEq, Builder)]
pub struct Extension {
    /// The qualified name of the extension element.
    name: String,
    /// The content of the extension element.
    value: Option<String>,
    /// The attributes for the extension element.
    attrs: HashMap<String, String>,
    /// The children of the extension element. A map of local names to child elements.
    children: HashMap<String, Vec<Extension>>,
}

impl Extension {
    /// Return the qualified name of this extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// extension.set_name("ext:name");
    /// assert_eq!(extension.name(), "ext:name");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the qualified name of this extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// extension.set_name("ext:name");
    /// ```
    pub fn set_name<V>(&mut self, name: V)
    where
        V: Into<String>,
    {
        self.name = name.into();
    }

    /// Return the text content of this extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// extension.set_value("John Doe".to_string());
    /// assert_eq!(extension.value(), Some("John Doe"));
    /// ```
    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|s| s.as_str())
    }

    /// Set the text content of this extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// extension.set_value("John Doe".to_string());
    /// ```
    pub fn set_value<V>(&mut self, value: V)
    where
        V: Into<Option<String>>,
    {
        self.value = value.into();
    }

    /// Return the attributes for the extension element.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// let mut attrs = HashMap::<String, String>::new();
    /// attrs.insert("email".to_string(), "johndoe@example.com".to_string());
    /// extension.set_attrs(attrs.clone());
    /// assert_eq!(*extension.attrs(), attrs);
    /// ```
    pub fn attrs(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    /// Set the attributes for the extension element.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// extension.set_attrs(HashMap::new());
    /// ```
    pub fn set_attrs<V>(&mut self, attrs: V)
    where
        V: Into<HashMap<String, String>>,
    {
        self.attrs = attrs.into();
    }

    /// Return the children of the extension element.
    ///
    /// A map of local names to child elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// let mut children = HashMap::<String, Vec<Extension>>::new();
    /// children.insert("ext:child".to_string(), Vec::new());
    /// extension.set_children(children);
    /// assert!(extension.children().contains_key("ext:child"));
    /// ```
    pub fn children(&self) -> &HashMap<String, Vec<Extension>> {
        &self.children
    }

    /// Set the children of the extension element.
    ///
    /// A map of local names to child elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// extension.set_children(HashMap::new());
    /// ```
    pub fn set_children<V>(&mut self, children: V)
    where
        V: Into<HashMap<String, Vec<Extension>>>,
    {
        self.children = children.into();
    }
}

impl ToXml for Extension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = self.name.as_bytes();
        let mut element = BytesStart::borrowed(name, name.len());
        element.extend_attributes(self.attrs.iter().map(|a| (a.0.as_bytes(), a.1.as_bytes())));
        writer.write_event(Event::Start(element))?;

        if let Some(value) = self.value.as_ref() {
            writer
                .write_event(Event::Text(BytesText::borrowed(value.as_bytes())))?;
        }

        for extension in self.children.values().flat_map(|extensions| extensions) {
            extension.to_xml(writer)?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(name)))?;
        Ok(())
    }
}
