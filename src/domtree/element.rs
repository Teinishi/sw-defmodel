use super::{Node, attributes::Attributes, has_children::HasChildren, utils::debug_utf8};
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Element {
    pub(crate) name: Vec<u8>,
    pub(crate) attributes: Attributes,
    pub(crate) children: Vec<Node>,
    pub(crate) is_self_closing: bool,
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
    pub(crate) fn new_empty(name: Vec<u8>) -> Self {
        Self {
            name,
            attributes: Attributes::default(),
            children: Vec::new(),
            is_self_closing: true,
        }
    }

    pub(crate) fn from_bytes_start(
        e: quick_xml::events::BytesStart<'_>,
        is_self_closing: bool,
    ) -> Self {
        Self {
            name: e.name().as_ref().into(),
            attributes: Attributes::new(e.attributes_raw()),
            children: Vec::new(),
            is_self_closing,
        }
    }

    pub(crate) fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
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
    fn children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.children
    }
}
