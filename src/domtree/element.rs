use super::{HasChildren, HasChildrenMut, Node, attributes::Attributes, errors::{AttrError, ParseError}};
use crate::utils::debug_utf8;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(PartialEq, Eq, Clone)]
pub struct Element {
    pub name: Vec<u8>,
    pub attributes: Attributes,
    pub children: Vec<Node>,
    pub is_self_closing: bool,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[expect(dead_code)]
        #[derive(Debug)]
        struct Element<'a> {
            name: &'a str,
            attributes: &'a Attributes,
            children: &'a Vec<Node>,
            is_self_closing: bool,
        }
        let tmp = Element {
            name: debug_utf8(&self.name),
            attributes: &self.attributes,
            children: &self.children,
            is_self_closing: self.is_self_closing,
        };
        write!(f, "{:?}", tmp)
    }
}

impl Element {
    pub fn new_empty<K: AsRef<[u8]>>(name: K) -> Self {
        Self {
            name: name.as_ref().into(),
            attributes: Attributes::default(),
            children: Vec::new(),
            is_self_closing: true,
        }
    }

    pub(crate) fn from_bytes_start(
        e: quick_xml::events::BytesStart<'_>,
        is_self_closing: bool,
    ) -> Result<Self, ParseError> {
        Ok(Self {
            name: e.name().as_ref().into(),
            attributes: Attributes::new(e.attributes_raw())?,
            children: Vec::new(),
            is_self_closing,
        })
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(b"<")?;
        writer.write_all(&self.name)?;
        self.attributes.write(writer)?;

        if self.is_self_closing && self.children.is_empty() {
            writer.write_all(b"/>")?;
            return Ok(());
        }

        writer.write_all(b">")?;
        for child in &self.children {
            child.write(writer)?;
        }
        writer.write_all(b"</")?;
        writer.write_all(&self.name)?;
        writer.write_all(b">")
    }
}

impl HasChildren for Element {
    fn children(&self) -> &Vec<Node> {
        &self.children
    }
}

impl HasChildrenMut for Element {
    fn children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.children
    }
}

impl HasChildren for &Element {
    fn children(&self) -> &Vec<Node> {
        &self.children
    }
}

impl HasChildren for &mut Element {
    fn children(&self) -> &Vec<Node> {
        &self.children
    }
}

impl HasChildrenMut for &mut Element {
    fn children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.children
    }
}

pub trait HasAttr {
    fn attr<K: AsRef<[u8]>, T, E>(&self, key: K) -> Result<T, AttrError>
    where
        T: FromStr<Err = E>,
        AttrError: From<E>;
}

pub trait HasAttrMut: HasAttr {
    fn set_attr<K: AsRef<[u8]>, T: Display>(&mut self, key: K, value: T);
}

impl HasAttr for Element {
    fn attr<K: AsRef<[u8]>, T, E>(&self, key: K) -> Result<T, AttrError>
    where
        T: FromStr<Err = E>,
        AttrError: From<E>,
    {
        self.attributes.get(key)
    }
}

impl HasAttrMut for Element {
    fn set_attr<K: AsRef<[u8]>, T: Display>(&mut self, key: K, value: T) {
        self.attributes.set(key, value);
    }
}

impl HasAttr for &Element {
    fn attr<K: AsRef<[u8]>, T, E>(&self, key: K) -> Result<T, AttrError>
    where
        T: FromStr<Err = E>,
        AttrError: From<E>,
    {
        self.attributes.get(key)
    }
}

impl HasAttr for &mut Element {
    fn attr<K: AsRef<[u8]>, T, E>(&self, key: K) -> Result<T, AttrError>
    where
        T: FromStr<Err = E>,
        AttrError: From<E>,
    {
        self.attributes.get(key)
    }
}

impl HasAttrMut for &mut Element {
    fn set_attr<K: AsRef<[u8]>, T: Display>(&mut self, key: K, value: T) {
        self.attributes.set(key, value);
    }
}
