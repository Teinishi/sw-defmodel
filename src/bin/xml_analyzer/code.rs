use super::node_info::NodeInfo;
use heck::{ToSnakeCase as _, ToUpperCamelCase as _};
use std::io;

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

pub(super) fn write_node_code<W: io::Write>(
    f: &mut W,
    node: &NodeInfo,
    target_label: &str,
) -> io::Result<()> {
    let struct_name = node.name.to_upper_camel_case();

    let mut dependencies: Vec<String> = Vec::new();

    // 属性定義
    writeln!(f, "define_tag! {{")?;
    writeln!(
        f,
        "    #[doc = \"Represents `<{}>` tag in {target_label}.\"]",
        &node.name
    )?;
    write!(f, "    struct {struct_name} {{")?;
    if !node.attributes.is_empty() {
        write!(f, "\n    ")?;
    }
    for (attr_name, attr_info) in node.attributes.get_items() {
        let name = to_snake_ident(attr_name, "_attr");
        write!(f, "    {attr_name:?}")?;
        if &name != attr_name {
            write!(f, " => {name}")?;
        }
        write!(f, ": {},\n    ", attr_info.type_str())?;
    }
    writeln!(f, "}}\n}}")?;

    // 子をリストとそれ以外に分類
    let mut children = Vec::new();
    let mut child_lists = Vec::new();
    for (c_name, child_info) in node.children.get_items() {
        let c = child_info.inner();
        if c.attributes.is_empty()
            && c.children.iter().count() == 1
            && c.children.iter().all(|(_, v)| v.is_multiple())
        {
            child_lists.push((c_name, child_info));
        } else if !child_info.is_multiple() {
            children.push((c_name, child_info));
        } else {
            //panic!("Unexpected element: {c_name}");
            children.push((c_name, child_info));
        }
    }

    // 通常の子要素
    if !children.is_empty() {
        writeln!(f)?;
        writeln!(f, "define_unique_children!({struct_name} {{")?;
        for (child_name, child_info) in children {
            if child_info.is_multiple() {
                continue;
            }
            let name = to_snake_ident(child_name, "_child");
            let s_name = child_name.to_upper_camel_case();
            write!(f, "    <{child_name}>")?;
            if &name != child_name {
                write!(f, "=> {name}")?;
            }
            writeln!(f, ": {s_name},")?;
            dependencies.push(s_name);
        }
        writeln!(f, "}});")?;
    }

    // リスト
    if !child_lists.is_empty() {
        writeln!(f)?;
        writeln!(f, "define_lists!({struct_name} {{")?;
        for (list_name, list_info) in child_lists {
            let (item_name, _item_info) = list_info.inner().children.iter().next().unwrap();
            let s_name = item_name.to_upper_camel_case();
            writeln!(f, "    <{list_name}>: [<{item_name}>: {s_name}],")?;
            dependencies.push(s_name);
        }
        writeln!(f, "}});")?;
    }

    Ok(())
}
