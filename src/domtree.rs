mod attributes;
mod element;
pub mod errors;
mod has_children;
mod node;

pub use attributes::{AttrSlot, Attributes};
pub use element::{Element, HasAttr, HasAttrMut};
pub use errors::{AttrError, ParseError};
pub use has_children::{HasChildren, HasChildrenMut};
pub use node::Node;
use quick_xml::{Reader, errors::IllFormedError, events::Event};
use std::{fmt::Debug, io::BufRead, path::Path};

#[derive(Clone, Default, Debug)]
pub struct Document {
    root: Vec<Node>,
}

impl Document {
    pub fn from_xml_str(s: &str) -> Result<Self, ParseError> {
        let mut reader = Reader::from_str(s);
        Self::from_xml_reader(&mut reader)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        let mut reader = Reader::from_file(path)?;
        Self::from_xml_reader(&mut reader)
    }

    pub fn from_xml_reader<R: BufRead>(reader: &mut Reader<R>) -> Result<Self, ParseError> {
        let mut builder = TreeBuilder::default();

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) => {
                    let el = Element::from_bytes_start(e, false)?;
                    builder.start_element(el);
                }
                Event::End(e) => {
                    builder.end_element(e.name().as_ref())?;
                }
                Event::Empty(e) => {
                    let el = Element::from_bytes_start(e, true)?;
                    builder.push_node(Node::Element(el));
                }
                Event::Text(e) => builder.push_node(Node::Text(e.into_inner().into_owned())),
                Event::CData(e) => builder.push_node(Node::CData(e.into_inner().into_owned())),
                Event::Comment(e) => builder.push_node(Node::Comment(e.into_inner().into_owned())),
                Event::Decl(e) => builder.push_node(Node::Decl((&e as &[u8]).to_vec())),
                Event::PI(e) => builder.push_node(Node::PI(e.into_inner().into_owned())),
                Event::DocType(e) => builder.push_node(Node::DocType(e.into_inner().into_owned())),
                Event::GeneralRef(e) => {
                    builder.push_node(Node::GeneralRef(e.into_inner().into_owned()))
                }

                Event::Eof => break,
            }
            buf.clear();
        }

        builder.finish()?;

        Ok(Self { root: builder.root })
    }

    /*pub fn find<K: AsRef<[u8]>>(&self, path: &[K]) -> Option<&Element> {
        let mut r: Option<&Element> = None;
        for k in path {
            if let Some(el) = r {
                r = Some(el.single_element_by_name(k).map(|e| e.0)?);
            } else {
                r = Some(self.single_element_by_name(k).map(|e| e.0)?);
            }
        }
        r
    }
    pub fn find_ensure<K: AsRef<[u8]>>(&mut self, root_tag: K, path: &[K]) -> &mut Element {
        let mut r = self.ensure_element(root_tag).0;
        for k in path {
            r = r.ensure_element(k).0;
        }
        r
    }*/

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for node in &self.root {
            node.write(writer)?;
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut out = Vec::new();
        self.write(&mut out)?;
        Ok(out)
    }
}

impl HasChildren for Document {
    fn children(&self) -> &Vec<Node> {
        &self.root
    }
}

impl HasChildrenMut for Document {
    fn children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.root
    }
}

#[derive(Default, Debug)]
struct TreeBuilder {
    root: Vec<Node>,
    stack: Vec<Element>,
}

impl TreeBuilder {
    fn start_element(&mut self, el: Element) {
        self.stack.push(el);
    }

    fn end_element(&mut self, name: &[u8]) -> Result<(), quick_xml::Error> {
        let el = self.stack.pop().ok_or_else(|| {
            quick_xml::Error::IllFormed(IllFormedError::UnmatchedEndTag(name_to_string(name)))
        })?;

        if el.name != name {
            return Err(quick_xml::Error::IllFormed(
                IllFormedError::MismatchedEndTag {
                    expected: name_to_string(&el.name),
                    found: name_to_string(name),
                },
            ));
        }

        self.push_node(Node::Element(el));
        Ok(())
    }

    fn push_node(&mut self, node: Node) {
        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(node);
        } else {
            self.root.push(node);
        }
    }

    fn finish(&mut self) -> Result<(), quick_xml::Error> {
        if let Some(el) = self.stack.last() {
            return Err(quick_xml::Error::IllFormed(IllFormedError::MissingEndTag(
                name_to_string(&el.name),
            )));
        }
        Ok(())
    }
}

fn name_to_string(name: &[u8]) -> String {
    String::from_utf8_lossy(name).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_helper<F>(input: &str, expected: &str, callback: F)
    where
        F: FnOnce(&mut Document),
    {
        let mut doc = Document::from_xml_str(input).expect("parse failed");

        callback(&mut doc);

        let out = doc.to_bytes().expect("write failed");
        if out != expected.as_bytes() {
            panic!(
                "assertion `out == expected` failed\nout:\n{}\nexpected:\n{}",
                crate::utils::debug_utf8(&out),
                expected
            );
        }
        assert_eq!(out, expected.as_bytes());
    }

    #[test]
    fn read() {
        let xml = concat!(
            "<?xml version='1.0'?>\n",
            "<!DOCTYPE root>\n",
            "<?pi some=\"thing\"?>\n",
            "<root a=\"1\" b='2' c=\"a&amp;b\">\n",
            "  text&amp;more\n",
            "  <!--comment-->\n",
            "  <![CDATA[cdata]]>\n",
            "  &entity;\n",
            "  <empty />\n",
            "  <empty2></empty2>\n",
            "  <child>inner</child>\n",
            "</root>\n",
        );
        test_helper(xml, xml, |_| {});
    }

    #[test]
    fn read_malformed_xml() {
        // 多少 malformed でも受け入れてほしいケース:
        // - 数字で始まる属性名
        // - 改行を含む属性値
        let xml = "<root 123=\"456\" b=\"line1\nline2&amp;hoge\"></root>";

        test_helper(xml, xml, |doc| {
            if let Some(Node::Element(el)) = doc.root.first() {
                assert_eq!(el.attr("123"), Ok(456));
                assert_eq!(el.attr("b"), Ok("line1\nline2&hoge".to_owned()));
            } else {
                panic!("cannot get root element")
            }
        });
    }

    #[test]
    fn insert_before() {
        test_helper(
            "<root><child>inner</child></root>",
            "<root><child2 /><child>inner</child></root>",
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                let (_, i) = root
                    .single_element_by_name("child")
                    .expect("cannot get <child>");
                root.insert_element_before(i, Element::new_empty("child2"));
            },
        );
    }

    #[test]
    fn insert_before_with_indent() {
        test_helper(
            concat!("<root>\n", "  <child>inner</child>\n", "</root>\n",),
            concat!(
                "<root>\n",
                "  <child2 />\n",
                "  <child>inner</child>\n",
                "</root>\n",
            ),
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                let (_, i) = root
                    .single_element_by_name("child")
                    .expect("cannot get <child>");
                root.insert_element_before(i, Element::new_empty("child2"));
            },
        );
    }

    #[test]
    fn insert_after() {
        test_helper(
            "<root><child>inner</child></root>",
            "<root><child>inner</child><child2 /></root>",
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                let (_, i) = root
                    .single_element_by_name("child")
                    .expect("cannot get <child>");
                root.insert_element_after(i, Element::new_empty("child2"));
            },
        );
    }

    #[test]
    fn insert_after_with_indent() {
        test_helper(
            concat!("<root>\n", "  <child>inner</child>\n", "</root>\n",),
            concat!(
                "<root>\n",
                "  <child>inner</child>\n",
                "  <child2 />\n",
                "</root>\n",
            ),
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                let (_, i) = root
                    .single_element_by_name("child")
                    .expect("cannot get <child>");
                root.insert_element_after(i, Element::new_empty("child2"));
            },
        );
    }

    #[test]
    fn remove() {
        test_helper(
            "<root><child>inner</child><child2 /></root>",
            "<root><child2 /></root>",
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                let (_, i) = root
                    .single_element_by_name("child")
                    .expect("cannot get <child>");
                root.remove_element(i);
            },
        );
    }

    #[test]
    fn remove_with_indent() {
        test_helper(
            concat!(
                "<root>\n",
                "  <child>inner</child>\n",
                "  <child2 />\n",
                "</root>\n",
            ),
            concat!("<root>\n", "  <child2 />\n", "</root>\n",),
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                let (_, i) = root
                    .single_element_by_name("child")
                    .expect("cannot get <child>");
                root.remove_element(i);
            },
        );
    }

    #[test]
    fn push_element() {
        test_helper(
            concat!("<root>\n", "  <child>inner</child>\n", "</root>\n",),
            concat!(
                "<root>\n",
                "  <child>inner</child>\n",
                "  <child2 />\n",
                "</root>\n",
            ),
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                root.push_element(Element::new_empty("child2"));
            },
        );
    }

    #[test]
    fn ensure_element_2() {
        test_helper(
            concat!("<root>\n", "  <child>inner</child>\n", "</root>\n",),
            concat!("<root>\n", "  <child>inner</child>\n", "</root>\n",),
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                root.ensure_element("child");
            },
        );
    }

    #[test]
    fn ensure_element_1() {
        test_helper(
            concat!("<root>\n", "  <child>inner</child>\n", "</root>\n",),
            concat!(
                "<root>\n",
                "  <child>inner</child>\n",
                "  <child2 />\n",
                "</root>\n",
            ),
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                root.ensure_element("child2");
            },
        );
    }

    #[test]
    fn update_attribute() {
        test_helper("<root abc=\"def\" />", "<root abc=\"ghi\" />", |doc| {
            let (root, _) = doc
                .single_element_by_name_mut("root")
                .expect("cannot get <root>");
            root.attributes.set_unescaped("abc", "ghi");
        });
    }

    #[test]
    fn add_attribute() {
        test_helper(
            "<root abc=\"def\" />",
            "<root abc=\"def\" 123=\"456\" />",
            |doc| {
                let (root, _) = doc
                    .single_element_by_name_mut("root")
                    .expect("cannot get <root>");
                root.attributes.set_unescaped("123", "456");
            },
        );
    }

    #[test]
    fn remove_attribute() {
        test_helper("<root abc=\"def\" />", "<root />", |doc| {
            let (root, _) = doc
                .single_element_by_name_mut("root")
                .expect("cannot get <root>");
            let abc = root
                .attributes
                .remove("abc")
                .expect("cannot remove attribute `abc`");
            assert_eq!(abc.value(), "def");
        });
    }

    #[test]
    fn errors() {
        let input = "<root abc />";
        let r = Document::from_xml_str(input);
        assert!(matches!(r, Err(ParseError::ExpectedEq)));

        let input = "<root abc=def />";
        let r = Document::from_xml_str(input);
        assert!(matches!(r, Err(ParseError::ExpectedQuote)));
    }
}
