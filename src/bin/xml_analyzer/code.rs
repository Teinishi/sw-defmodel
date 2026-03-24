use super::{
    code_rule::{ChildClassificcation, CodeRule, NamePath, TypeDefinition},
    node_info::{NodeInfo, ValueType},
};
use heck::{ToSnakeCase as _, ToUpperCamelCase as _};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::{self, Write as _},
    hash::Hash,
};

// Rust で識別子に使用できないキーワードのリスト
const RUST_KEYWORDS: [&str; 50] = [
    "as", "async", "await", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

// XML属性名等を Rust の識別子にする
fn to_snake_ident(val: &str, fix: &str) -> String {
    let mut v = val.to_snake_case();
    if v.starts_with(|c: char| c.is_ascii_digit()) {
        v = format!("{fix}_{v}");
    }
    if RUST_KEYWORDS.contains(&v.as_str()) {
        v = format!("{v}_{fix}");
    }
    v
}

pub(super) fn write_code<R: CodeRule>(node: &NodeInfo, rule: &mut R) -> String {
    let mut f1 = String::new();
    let mut f2 = String::new();

    let mut nodes = BTreeMap::new();
    let mut reg = NameRegistory::default();
    let root_id = create_node_definitions(
        &mut nodes,
        &mut reg,
        NamePath::new(node.name.clone()),
        node,
        rule,
    );

    // nodes から重複を取り除く
    let mut merge_ids: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut remove_ids = HashSet::new();
    for (i, a) in &nodes {
        if remove_ids.contains(i) {
            continue;
        }
        for (j, b) in nodes.range(i + 1..) {
            if remove_ids.contains(j) {
                continue;
            }
            if a.struct_eq(b, &nodes).unwrap() {
                merge_ids.entry(*i).or_default().insert(*j);
                remove_ids.insert(*j);
            }
        }
    }
    for i in remove_ids {
        nodes.remove(&i);
        reg.name_paths.remove(&i);
    }

    // 消した name を復活させる
    let mut names = reg.finalize();
    for (i, s) in merge_ids {
        for j in s {
            names.insert(j, names.get(&i).unwrap().clone());
        }
    }

    // ドキュメントの定義を書く
    let root_name = names.get(&root_id).unwrap();
    writeln!(&mut f2, "define_root! {{").unwrap();
    writeln!(&mut f2, "    #[doc = \"Represents {}.\"]", R::TARGET_LABEL).unwrap();
    writeln!(&mut f2, "    struct {}Document {{", root_name).unwrap();
    writeln!(&mut f2, "        <{}> => {}", node.name, root_name).unwrap();
    writeln!(&mut f2, "    }}").unwrap();
    writeln!(&mut f2, "}}").unwrap();
    writeln!(&mut f2).unwrap();

    // 各要素の定義マクロを書く
    for node_def in nodes.values() {
        node_def
            .write_code(&mut f2, R::TARGET_LABEL, &names)
            .unwrap();
        writeln!(&mut f2).unwrap();
    }

    rule.finalize(&mut f1, &mut f2).unwrap();

    f1.push_str(&f2);
    f1
}

fn create_node_definitions<R: CodeRule>(
    nodes: &mut BTreeMap<usize, NodeDefinition>,
    reg: &mut NameRegistory,
    path: NamePath,
    node: &NodeInfo,
    rule: &mut R,
) -> usize {
    let name_id = reg.register_name(path.clone());

    let attributes = node
        .attributes
        .get_items()
        .into_iter()
        .map(|(attr_name, attr_info)| {
            let name = to_snake_ident(attr_name, "attr");
            AttributeDefinition {
                xml_name: attr_name.clone(),
                name,
                ty: *attr_info.ty(),
            }
        })
        .collect();

    let mut children = Vec::new();
    let mut lists = Vec::new();
    for (c_name, child_info) in node.children.get_items() {
        let c_path = path.join(c_name.to_owned());
        let cls = rule.override_child(&c_path, child_info).unwrap_or_else(|| {
            let c = child_info.inner();
            if c.attributes.is_empty()
                && c.children.iter().count() == 1
                && c.children.iter().all(|(_, v)| v.is_multiple())
            {
                ChildClassificcation::list()
            } else if !child_info.is_multiple() {
                ChildClassificcation::unique()
            } else {
                panic!("Unexpected element: {c_path}");
            }
        });

        match cls {
            ChildClassificcation::Unique { name, ty } => children.push(ChildDefinition {
                xml_name: c_name.clone(),
                name: name.unwrap_or_else(|| to_snake_ident(c_name, "el")),
                ty: ty.unwrap_or_else(|| {
                    TypeDefinition::Registered(create_node_definitions(
                        nodes,
                        reg,
                        c_path,
                        child_info.inner(),
                        rule,
                    ))
                }),
            }),
            ChildClassificcation::List { list_name, item_ty } => {
                let (item_xml_name, item_info) = child_info.inner().children.iter().next().unwrap();
                lists.push(ListDefinition {
                    list_xml_name: c_name.clone(),
                    list_name: list_name.unwrap_or_else(|| to_snake_ident(c_name, "list")),
                    item_xml_name: item_xml_name.clone(),
                    item_ty: item_ty.unwrap_or_else(|| {
                        TypeDefinition::Registered(create_node_definitions(
                            nodes,
                            reg,
                            c_path.join(item_xml_name.to_owned()),
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
            xml_name: path.name().to_owned(),
            name_id,
            attributes,
            children,
            lists,
        },
    );

    name_id
}

fn type_str<'a>(ty: &TypeDefinition, names: &'a BTreeMap<usize, String>) -> &'a str {
    match ty {
        TypeDefinition::Inline(s) => s,
        TypeDefinition::Registered(idx) => names.get(idx).unwrap(),
    }
}

#[derive(Default, Debug)]
struct NameRegistory {
    name_paths: BTreeMap<usize, NamePath>,
}

impl NameRegistory {
    fn register_name(&mut self, path: NamePath) -> usize {
        let id = self.name_paths.len();
        self.name_paths.insert(id, path);
        id
    }

    fn finalize(&self) -> BTreeMap<usize, String> {
        self.name_paths
            .iter()
            .map(|(id, name_path)| {
                let len = name_path.len();
                'size: for size in 1..len {
                    // 1文字の要素が先頭に来ないように
                    if name_path.as_slice()[len - size].len() <= 1 {
                        continue;
                    }
                    let s = name_path.tail(size);
                    for (i, other) in self.name_paths.iter() {
                        if i != id && other.tail(size) == s {
                            continue 'size;
                        }
                    }
                    return (*id, s.join(" ").to_upper_camel_case());
                }
                (*id, name_path.join_str(" ").to_upper_camel_case())
            })
            .collect()
    }
}

fn hashset_eq<T: Eq + Hash>(a: &[T], b: &[T]) -> bool {
    a.len() == b.len() && a.iter().collect::<HashSet<_>>() == b.iter().collect::<HashSet<_>>()
}

fn sort_eq<T: StructEq, F>(
    a: &[T],
    b: &[T],
    mut compare: F,
    nodes: &BTreeMap<usize, NodeDefinition>,
) -> Option<bool>
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    if a.len() != b.len() {
        return Some(false);
    }
    let mut ra: Vec<&T> = a.iter().collect();
    let mut rb: Vec<&T> = b.iter().collect();
    ra.sort_by(|va, vb| compare(*va, *vb));
    rb.sort_by(|va, vb| compare(*va, *vb));
    ra.iter().zip(rb.iter()).try_fold(true, |acc, (va, vb)| {
        va.struct_eq(vb, nodes).map(|e| acc && e)
    })
}

trait StructEq {
    fn struct_eq(&self, other: &Self, nodes: &BTreeMap<usize, NodeDefinition>) -> Option<bool>;
}

impl StructEq for TypeDefinition {
    fn struct_eq(&self, other: &Self, nodes: &BTreeMap<usize, NodeDefinition>) -> Option<bool> {
        match (self, other) {
            (Self::Inline(a), Self::Inline(b)) => Some(a == b),
            (Self::Registered(a), Self::Registered(b)) => {
                nodes.get(a)?.struct_eq(nodes.get(b)?, nodes)
            }
            _ => Some(false),
        }
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
    fn write_code<W: fmt::Write>(
        &self,
        f: &mut W,
        target_label: &str,
        names: &BTreeMap<usize, String>,
    ) -> fmt::Result {
        let struct_name = names.get(&self.name_id).unwrap();

        // 属性と子要素で名前が被った場合の処理
        let mut name_count: HashMap<&String, usize> =
            HashMap::with_capacity(self.attributes.len() + self.children.len() + self.lists.len());
        for attr in &self.attributes {
            *name_count.entry(&attr.name).or_insert(0) += 1;
        }
        for child in &self.children {
            *name_count.entry(&child.name).or_insert(0) += 1;
        }
        for list in &self.lists {
            *name_count.entry(&list.list_name).or_insert(0) += 1;
        }

        // 属性定義
        writeln!(f, "define_tag! {{")?;
        writeln!(
            f,
            "    #[doc = \"Represents `<{}>` tag in {target_label}.\"]",
            self.xml_name,
        )?;
        if self.attributes.is_empty() && self.children.is_empty() && self.lists.is_empty() {
            writeln!(f, "    #[expect(dead_code)]")?;
        }
        write!(f, "    struct {struct_name} {{")?;
        if !self.attributes.is_empty() {
            write!(f, "\n    ")?;
        }
        for attr in &self.attributes {
            let name = if name_count.get(&attr.name).is_some_and(|c| *c > 1) {
                &format!("{}_attr", attr.name)
            } else {
                &attr.name
            };

            write!(f, "    {:?}", attr.xml_name)?;
            if &attr.xml_name != name {
                write!(f, " => {}", name)?;
            }
            write!(f, ": {},\n    ", attr.ty.as_str())?;
        }
        writeln!(f, "}}")?;
        writeln!(f, "}}")?;

        // 通常の子要素
        if !self.children.is_empty() {
            writeln!(f, "define_unique_children!({struct_name} {{")?;
            for child in &self.children {
                let name = if name_count.get(&child.name).is_some_and(|c| *c > 1) {
                    &format!("{}_el", child.name)
                } else {
                    &child.name
                };

                write!(f, "    <{}>", child.xml_name)?;
                if &child.xml_name != name {
                    write!(f, " => {}", name)?;
                }
                writeln!(f, ": {},", type_str(&child.ty, names))?;
            }
            writeln!(f, "}});")?;
        }

        // リスト
        if !self.lists.is_empty() {
            writeln!(f, "define_lists!({struct_name} {{")?;
            for list in &self.lists {
                let name = if name_count.get(&list.list_name).is_some_and(|c| *c > 1) {
                    &format!("{}_list", list.list_name)
                } else {
                    &list.list_name
                };

                write!(f, "    <{}>", list.list_xml_name)?;
                if &list.list_xml_name != name {
                    write!(f, " => {}", name)?;
                }
                writeln!(
                    f,
                    ": [<{}>: {}],",
                    list.item_xml_name,
                    type_str(&list.item_ty, names)
                )?;
            }
            writeln!(f, "}});")?;
        }

        Ok(())
    }
}

impl StructEq for NodeDefinition {
    fn struct_eq(&self, other: &Self, nodes: &BTreeMap<usize, NodeDefinition>) -> Option<bool> {
        if self.xml_name != other.xml_name
            || self.attributes.len() != other.attributes.len()
            || self.children.len() != other.children.len()
            || self.lists.len() != other.lists.len()
        {
            return Some(false);
        }
        Some(
            hashset_eq(&self.attributes, &other.attributes)
                && sort_eq(
                    &self.children,
                    &other.children,
                    |a, b| a.xml_name.cmp(&b.xml_name),
                    nodes,
                )?
                && sort_eq(
                    &self.lists,
                    &other.lists,
                    |a, b| a.item_xml_name.cmp(&b.list_xml_name),
                    nodes,
                )?,
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct AttributeDefinition {
    xml_name: String,
    name: String,
    ty: ValueType,
}

#[derive(Debug)]
struct ChildDefinition {
    xml_name: String,
    name: String,
    ty: TypeDefinition,
}

impl StructEq for ChildDefinition {
    fn struct_eq(&self, other: &Self, nodes: &BTreeMap<usize, NodeDefinition>) -> Option<bool> {
        Some(
            self.xml_name == other.xml_name
                && self.name == other.name
                && self.ty.struct_eq(&other.ty, nodes)?,
        )
    }
}

#[derive(Debug)]
struct ListDefinition {
    list_xml_name: String,
    list_name: String,
    item_xml_name: String,
    item_ty: TypeDefinition,
}

impl StructEq for ListDefinition {
    fn struct_eq(&self, other: &Self, nodes: &BTreeMap<usize, NodeDefinition>) -> Option<bool> {
        Some(
            self.list_xml_name == other.list_xml_name
                && self.list_name == other.list_name
                && self.item_xml_name == other.item_xml_name
                && self.item_ty.struct_eq(&other.item_ty, nodes)?,
        )
    }
}
