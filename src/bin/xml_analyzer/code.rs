use super::{
    code_rule::{ChildClassificcation, CodeRule, TypeDefinition},
    node_info::NodeInfo,
};
use heck::{ToSnakeCase as _, ToUpperCamelCase as _};
use std::{collections::BTreeMap, io, ops::Index};

const RUST_KEYWORDS: [&str; 50] = [
    "as", "async", "await", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

fn to_snake_ident(val: &str, suffix: &str) -> String {
    let mut v = val.to_snake_case();
    if RUST_KEYWORDS.contains(&v.as_str()) {
        v.push_str(suffix);
        v = v.to_snake_case();
    }
    v
}

pub(super) fn write_code<W: io::Write, R: CodeRule>(
    f: &mut W,
    node: &NodeInfo,
    rule: &mut R,
) -> io::Result<()> {
    let mut nodes = BTreeMap::new();
    let mut reg = NameRegistory::default();
    create_node_definitions(
        &mut nodes,
        &mut reg,
        &NamePath::default(),
        &node.name,
        node,
        rule,
    );

    let names = reg.finalize();

    for node_def in nodes.values() {
        node_def.write_code(f, R::TARGET_LABEL, &names)?;
    }

    Ok(())
}

#[derive(Default, Debug)]
struct NameRegistory {
    names: Vec<NamePath>,
}

impl NameRegistory {
    fn register_name(&mut self, path: NamePath) -> usize {
        self.names.push(path);
        self.names.len() - 1
    }

    fn finalize(&self) -> Vec<String> {
        self.names
            .iter()
            .enumerate()
            .map(|(idx, name_path)| {
                let len = name_path.path.len();
                'size: for size in 1..len {
                    let s = &name_path.path[len - size..];
                    if size < len {
                        for (i, other) in self.names.iter().enumerate() {
                            if i != idx && &other.path[other.path.len().saturating_sub(size)..] == s
                            {
                                continue 'size;
                            }
                        }
                    }
                    return s.join(" ").to_upper_camel_case();
                }
                name_path.path.join(" ").to_upper_camel_case()
            })
            .collect()
    }
}

#[derive(Clone, Default, Debug)]
struct NamePath {
    path: Vec<String>,
}

impl NamePath {
    fn clone_push(&self, value: String) -> Self {
        let mut s = self.clone();
        s.path.push(value);
        s
    }
}

#[derive(Debug)]
struct NodeDefinition {
    xml_name: String,
    name_id: usize,
    attributes: Vec<AttributeDefinition>,
    children: Vec<ChildDefinition>,
    lists: Vec<ListDefinition>,
}

impl NodeDefinition {
    fn write_code<W: io::Write>(
        &self,
        f: &mut W,
        target_label: &str,
        names: &[String],
    ) -> io::Result<()> {
        let struct_name = names.index(self.name_id);

        // 属性定義
        writeln!(f, "define_tag! {{")?;
        writeln!(
            f,
            "    #[doc = \"Represents `<{}>` tag in {target_label}.\"]",
            self.xml_name,
        )?;
        write!(f, "    struct {struct_name} {{")?;
        if !self.attributes.is_empty() {
            write!(f, "\n    ")?;
        }
        for attr in &self.attributes {
            write!(f, "    {:?}", attr.xml_name)?;
            if attr.xml_name != attr.name {
                write!(f, " => {}", attr.name)?;
            }
            write!(f, ": {},\n    ", type_str(&attr.ty, names))?;
        }
        writeln!(f, "}}\n}}\\n")?;

        // 通常の子要素
        if !self.children.is_empty() {
            writeln!(f, "define_unique_children!({struct_name} {{")?;
            for child in &self.children {
                write!(f, "    <{}>", child.xml_name)?;
                if child.xml_name != child.name {
                    write!(f, "=> {}", child.name)?;
                }
                writeln!(f, ": {},", type_str(&child.ty, names))?;
            }
            writeln!(f, "}});")?;
        }

        // リスト
        if !self.lists.is_empty() {
            writeln!(f, "define_lists!({struct_name} {{")?;
            for list in &self.lists {
                writeln!(
                    f,
                    "    <{}>: [<{}>: {}],",
                    list.list_xml_name,
                    list.item_xml_name,
                    type_str(&list.item_ty, names)
                )?;
            }
            writeln!(f, "}});")?;
        }

        Ok(())
    }
}

fn type_str<'a>(ty: &TypeDefinition, names: &'a [String]) -> &'a str {
    match ty {
        TypeDefinition::Inline(s) => s,
        TypeDefinition::Registered(idx) => names.index(*idx),
    }
}

#[derive(Debug)]
struct AttributeDefinition {
    xml_name: String,
    name: String,
    ty: TypeDefinition,
}

#[derive(Debug)]
struct ChildDefinition {
    xml_name: String,
    name: String,
    ty: TypeDefinition,
}

#[derive(Debug)]
struct ListDefinition {
    list_xml_name: String,
    list_name: String,
    item_xml_name: String,
    item_ty: TypeDefinition,
}

fn create_node_definitions<R: CodeRule>(
    nodes: &mut BTreeMap<usize, NodeDefinition>,
    reg: &mut NameRegistory,
    parent_path: &NamePath,
    name: &str,
    node: &NodeInfo,
    rule: &mut R,
) -> usize {
    let path = parent_path.clone_push(name.to_owned());
    let name_id = reg.register_name(path.clone());

    let attributes = node
        .attributes
        .get_items()
        .into_iter()
        .map(|(attr_name, attr_info)| {
            let name = to_snake_ident(attr_name, "_attr");
            AttributeDefinition {
                xml_name: attr_name.clone(),
                name,
                ty: TypeDefinition::Inline(attr_info.type_str()),
            }
        })
        .collect();

    let mut children = Vec::new();
    let mut lists = Vec::new();
    for (c_name, child_info) in node.children.get_items() {
        let cls = rule.override_child(c_name, child_info).unwrap_or_else(|| {
            let c = child_info.inner();
            if c.attributes.is_empty()
                && c.children.iter().count() == 1
                && c.children.iter().all(|(_, v)| v.is_multiple())
            {
                ChildClassificcation::list()
            } else if !child_info.is_multiple() {
                ChildClassificcation::unique()
            } else {
                panic!("Unexpected element: {c_name}");
            }
        });

        match cls {
            ChildClassificcation::Unique { name, ty } => children.push(ChildDefinition {
                xml_name: c_name.clone(),
                name: name.unwrap_or_else(|| to_snake_ident(c_name, "_el")),
                ty: ty.unwrap_or_else(|| {
                    TypeDefinition::Registered(create_node_definitions(
                        nodes,
                        reg,
                        &path,
                        c_name,
                        child_info.inner(),
                        rule,
                    ))
                }),
            }),
            ChildClassificcation::List { list_name, item_ty } => {
                let list_path = path.clone_push(c_name.to_owned());
                let (item_xml_name, item_info) = child_info.inner().children.iter().next().unwrap();
                lists.push(ListDefinition {
                    list_xml_name: c_name.clone(),
                    list_name: list_name.unwrap_or_else(|| to_snake_ident(c_name, "_list")),
                    item_xml_name: item_xml_name.clone(),
                    item_ty: item_ty.unwrap_or_else(|| {
                        TypeDefinition::Registered(create_node_definitions(
                            nodes,
                            reg,
                            &list_path,
                            item_xml_name,
                            item_info.inner(),
                            rule,
                        ))
                    }),
                });
            }
        }
    }

    nodes.insert(
        name_id,
        NodeDefinition {
            xml_name: name.to_owned(),
            name_id,
            attributes,
            children,
            lists,
        },
    );

    name_id
}
