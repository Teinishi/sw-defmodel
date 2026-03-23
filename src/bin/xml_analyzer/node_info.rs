use super::ordered_map::OrderedMap;
use quick_xml::{Reader, events::Event};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashMap},
    io::{self, BufRead},
    path::Path,
};

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub(super) enum ValueType {
    Bool,
    U32,
    U64,
    I32,
    F32,
    String,
}

impl ValueType {
    pub(super) fn as_str(&self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I32 => "i32",
            Self::F32 => "f32",
            Self::String => "String",
        }
    }
}

pub(super) fn infer_type(s: &str) -> ValueType {
    if s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("false") {
        return ValueType::Bool;
    }
    if s.parse::<u32>().is_ok() {
        return ValueType::U32;
    }
    if s.parse::<u64>().is_ok() {
        return ValueType::U64;
    }
    if s.parse::<i32>().is_ok() {
        return ValueType::I32;
    }
    if s.parse::<f32>().is_ok() {
        return ValueType::F32;
    }
    ValueType::String
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub(super) struct AttributeInfo {
    types: BTreeSet<ValueType>,
}

impl AttributeInfo {
    fn add(&mut self, ty: ValueType) {
        self.types.insert(ty);
    }

    fn merge(&mut self, other: Self) {
        self.types.extend(other.types);
    }

    pub(super) fn ty(&self) -> &ValueType {
        self.types.last().unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub(super) struct ChildInfo {
    max_count: usize,
    node: Box<NodeInfo>,
}

impl ChildInfo {
    pub(super) fn is_multiple(&self) -> bool {
        self.max_count > 1
    }

    pub(super) fn inner(&self) -> &NodeInfo {
        &self.node
    }

    pub(super) fn merge(&mut self, other: Self) {
        self.max_count = self.max_count.max(other.max_count);
        self.node.merge(*other.node);
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub(super) struct NodeInfo {
    pub(super) name: String,
    pub(super) attributes: OrderedMap<String, AttributeInfo>,
    pub(super) children: OrderedMap<String, ChildInfo>,
}

impl NodeInfo {
    pub(super) fn merge(&mut self, other: Self) {
        if self.name.is_empty() {
            self.name = other.name;
        } else if self.name != other.name {
            panic!(
                "Unable to merge NodeInfo: {:?} and {:?}",
                self.name, other.name
            );
        }
        self.attributes
            .merge_with(other.attributes, |a, b| a.merge(b));
        self.children.merge_with(other.children, |a, b| a.merge(b));
    }
}

pub(super) fn analyze_reader<R: BufRead>(reader: &mut Reader<R>) -> NodeInfo {
    reader.config_mut().expand_empty_elements = true;

    let mut buf = Vec::new();
    let mut stack: Vec<NodeInfo> = Vec::new();
    let mut root = None;

    // 同一親内での子出現回数カウント
    let mut child_count_stack: Vec<HashMap<String, usize>> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                let mut node = NodeInfo {
                    name: name.clone(),
                    ..Default::default()
                };

                // 属性
                node.attributes.begin_sequence();
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let value = attr.unescape_value().unwrap().to_string();

                    let ty = infer_type(&value);

                    node.attributes
                        .entry_or_insert_with(key.clone(), AttributeInfo::default)
                        .add(ty);
                }
                node.attributes.end_sequence();

                // 親の child_count 更新
                if let Some(map) = child_count_stack.last_mut() {
                    *map.entry(name.clone()).or_insert(0) += 1;
                }

                node.children.begin_sequence();
                stack.push(node);
                child_count_stack.push(HashMap::new());
            }

            Ok(Event::End(_)) => {
                let mut node = stack.pop().unwrap();
                let child_counts = child_count_stack.pop().unwrap();

                node.children.end_sequence();
                for (c_name, c) in node.children.iter_mut() {
                    c.max_count = *child_counts.get(c_name).unwrap();
                }

                if let Some(parent) = stack.last_mut() {
                    let entry = parent
                        .children
                        .entry_or_insert_with(node.name.clone(), ChildInfo::default);

                    entry.node.merge(node);
                } else {
                    root = Some(node);
                }
            }

            Ok(Event::Eof) => break,
            Err(e) => panic!("Error: {:?}", e),
            _ => {}
        }

        buf.clear();
    }

    root.unwrap()
}

pub(super) fn analyze_files<P: AsRef<Path> + Sync>(paths: &[P]) -> NodeInfo {
    use rayon::prelude::*;

    paths
        .par_iter()
        .map(|path| analyze_reader(&mut Reader::from_file(path).unwrap()))
        .reduce(NodeInfo::default, |mut a: NodeInfo, b: NodeInfo| {
            a.merge(b);
            a
        })
}

#[allow(dead_code)]
pub(super) fn print_node<W: io::Write>(
    f: &mut W,
    node: &NodeInfo,
    indent: usize,
) -> io::Result<()> {
    let pad = " ".repeat(indent);

    writeln!(f, "{}", node.name)?;

    for (k, v) in node.attributes.get_items() {
        writeln!(f, "{}  Attr: {} -> {:?}", pad, k, v.types)?;
    }

    for (_, v) in node.children.get_items() {
        write!(f, "{}  Child", pad)?;
        if v.is_multiple() {
            write!(f, " (max: {})", v.max_count)?;
        }
        write!(f, ": ")?;
        print_node(f, &v.node, indent + 2)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_order() {
        let input = r#"<definition name="Block" category="0" type="0" mass="1.000000" />"#;
        let node_info = analyze_reader(&mut Reader::from_str(input));
        assert_eq!(
            node_info.attributes.get_keys(),
            vec!["name", "category", "type", "mass"]
        );
    }
}
