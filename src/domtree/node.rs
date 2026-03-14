use super::{Element, utils::debug_utf8};
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum Node {
    Element(Element),
    Text(Vec<u8>),
    CData(Vec<u8>),
    Comment(Vec<u8>),
    Decl(Vec<u8>),
    PI(Vec<u8>),
    DocType(Vec<u8>),
    GeneralRef(Vec<u8>),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[expect(dead_code)]
        #[derive(Debug)]
        enum Node<'a> {
            Element(&'a Element),
            Text(&'a str),
            CData(&'a str),
            Comment(&'a str),
            Decl(&'a str),
            PI(&'a str),
            DocType(&'a str),
            GeneralRef(&'a str),
        }
        let tmp = match self {
            Self::Element(el) => Node::Element(el),
            Self::Text(bytes) => Node::Text(debug_utf8(bytes)),
            Self::CData(bytes) => Node::CData(debug_utf8(bytes)),
            Self::Comment(bytes) => Node::Comment(debug_utf8(bytes)),
            Self::Decl(bytes) => Node::Decl(debug_utf8(bytes)),
            Self::PI(bytes) => Node::PI(debug_utf8(bytes)),
            Self::DocType(bytes) => Node::DocType(debug_utf8(bytes)),
            Self::GeneralRef(bytes) => Node::GeneralRef(debug_utf8(bytes)),
        };
        write!(f, "{:?}", tmp)
    }
}

impl Node {
    pub(crate) fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Node::Element(el) => el.write(writer),
            Node::Text(bytes) => writer.write_all(bytes),
            Node::CData(bytes) => {
                writer.write_all(b"<![CDATA[")?;
                writer.write_all(bytes)?;
                writer.write_all(b"]]>")
            }
            Node::Comment(bytes) => {
                writer.write_all(b"<!--")?;
                writer.write_all(bytes)?;
                writer.write_all(b"-->")
            }
            Node::Decl(bytes) => {
                writer.write_all(b"<?")?;
                writer.write_all(bytes)?;
                writer.write_all(b"?>")
            }
            Node::PI(bytes) => {
                writer.write_all(b"<?")?;
                writer.write_all(bytes)?;
                writer.write_all(b"?>")
            }
            Node::DocType(bytes) => {
                // quick_xml の DocType は `<!DOCTYPE` とそれに続く空白を含まず、
                // 実質的に "root ..." の部分だけが入ってくるので、ここでは 1 つ空白を挿入する。
                writer.write_all(b"<!DOCTYPE ")?;
                writer.write_all(bytes)?;
                writer.write_all(b">")
            }
            Node::GeneralRef(bytes) => {
                writer.write_all(b"&")?;
                writer.write_all(bytes)?;
                writer.write_all(b";")
            }
        }
    }
}
