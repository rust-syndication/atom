use std::collections::BTreeMap;
use std::io::Write;
use std::str;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;

use crate::error::XmlError;
use crate::toxml::ToXml;

pub(crate) mod util;

/// A map of extension namespace prefixes to local names to elements.
pub type ExtensionMap = BTreeMap<String, BTreeMap<String, Vec<Extension>>>;

/// A namespaced extension.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct Extension {
    /// The qualified name of the extension element.
    pub name: String,
    /// The content of the extension element.
    pub value: Option<String>,
    /// The attributes for the extension element.
    #[cfg_attr(feature = "builders", builder(setter(each = "attr")))]
    pub attrs: BTreeMap<String, String>,
    /// The children of the extension element. A map of local names to child elements.
    #[cfg_attr(feature = "builders", builder(setter(each = "child")))]
    pub children: BTreeMap<String, Vec<Extension>>,
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
        self.value.as_deref()
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
    /// use std::collections::BTreeMap;
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// let mut attrs = BTreeMap::<String, String>::new();
    /// attrs.insert("email".to_string(), "johndoe@example.com".to_string());
    /// extension.set_attrs(attrs.clone());
    /// assert_eq!(*extension.attrs(), attrs);
    /// ```
    pub fn attrs(&self) -> &BTreeMap<String, String> {
        &self.attrs
    }

    /// Set the attributes for the extension element.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// extension.set_attrs(BTreeMap::new());
    /// ```
    pub fn set_attrs<V>(&mut self, attrs: V)
    where
        V: Into<BTreeMap<String, String>>,
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
    /// use std::collections::BTreeMap;
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// let mut children = BTreeMap::<String, Vec<Extension>>::new();
    /// children.insert("ext:child".to_string(), Vec::new());
    /// extension.set_children(children);
    /// assert!(extension.children().contains_key("ext:child"));
    /// ```
    pub fn children(&self) -> &BTreeMap<String, Vec<Extension>> {
        &self.children
    }

    /// Set the children of the extension element.
    ///
    /// A map of local names to child elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use atom_syndication::extension::Extension;
    ///
    /// let mut extension = Extension::default();
    /// extension.set_children(BTreeMap::new());
    /// ```
    pub fn set_children<V>(&mut self, children: V)
    where
        V: Into<BTreeMap<String, Vec<Extension>>>,
    {
        self.children = children.into();
    }
}

impl ToXml for Extension {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let mut element = BytesStart::new(&self.name);
        element.extend_attributes(self.attrs.iter().map(|a| (a.0.as_bytes(), a.1.as_bytes())));
        writer
            .write_event(Event::Start(element))
            .map_err(XmlError::new)?;

        if let Some(value) = self.value.as_ref() {
            writer
                .write_event(Event::Text(BytesText::new(value)))
                .map_err(XmlError::new)?;
        }

        for extension in self.children.values().flatten() {
            extension.to_xml(writer)?;
        }

        writer
            .write_event(Event::End(BytesEnd::new(&self.name)))
            .map_err(XmlError::new)?;
        Ok(())
    }
}

#[cfg(feature = "builders")]
impl ExtensionBuilder {
    /// Builds a new `Extension`.
    pub fn build(&self) -> Extension {
        self.build_impl().unwrap()
    }
}
