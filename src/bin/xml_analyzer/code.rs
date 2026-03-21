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

    writeln!(f, "define_tag! {{")?;
    writeln!(
        f,
        "    #[doc = \"Represents `<{}>` tag in {}.\"]",
        &node.name, target_label
    )?;
    write!(f, "    struct {} {{", struct_name)?;
    if !node.attributes.is_empty() {
        write!(f, "\n    ")?;
    }
    for (attr_name, attr_info) in node.attributes.get_items() {
        let name = to_snake_ident(attr_name, "_attr");
        write!(f, "    {:?}", attr_name)?;
        if &name != attr_name {
            write!(f, " => {}", name)?;
        }
        write!(f, ": {},\n    ", attr_info.type_str())?;
    }
    writeln!(f, "}}\n}}")?;

    Ok(())
}
