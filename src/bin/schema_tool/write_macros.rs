use super::{
    schema_analyzer::{SchemaAttribute, SchemaChild},
    write_rule::SchemaWriteRule,
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use std::io::{self, BufWriter, Write};

const RUST_KEYWORDS: [&str; 50] = [
    "as", "async", "await", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

pub(super) trait WriteWithIndent {
    fn write<W: Write>(&self, f: &mut W, indent: &str) -> io::Result<()>;
}

// define_tag マクロで属性定義
pub(super) fn write_define_tag<W: Write, R: SchemaWriteRule>(
    f: &mut BufWriter<W>,
    tag_name: &str,
    name: &str,
    attributes: Vec<SchemaAttribute>,
    rule: &mut R,
) -> io::Result<()> {
    writeln!(f, "define_tag! {{")?;
    writeln!(
        f,
        "    #[doc = \"Represents `<{tag_name}>` tag in {}.\"]",
        R::TARGET_LABEL
    )?;
    writeln!(f, "    struct {name} {{")?;

    for attr in attributes {
        let override_type = rule.before_define_attribute(tag_name, &attr);
        let key = attr.get_key();

        if let Some(doc) = override_type.doc {
            writeln!(f, "        #[doc = {:?}]", doc)?;
        }

        if RUST_KEYWORDS.contains(&key.as_str()) {
            write!(f, "        {:?} => {}_attr: ", key, key)?;
        } else {
            write!(f, "        {:?}: ", key)?;
        }

        let value_type = override_type
            .val_type
            .unwrap_or_else(|| attr.get_value_type(R::MAX_ENUM));
        value_type.write(f, "        ")?;
        writeln!(f, ",")?;
    }

    writeln!(f, "    }}")?;
    writeln!(f, "}}")
}

// define_unique_children マクロで親と子要素の紐づけ定義
pub(super) fn write_define_unique_children<W: Write>(
    f: &mut BufWriter<W>,
    name: &str,
    children: &[(SchemaChild, Option<&'static str>)],
) -> io::Result<()> {
    writeln!(f, "define_unique_children!({} {{", name)?;

    for (child, type_name) in children {
        let child_name = child.get_name().to_snake_case();
        if RUST_KEYWORDS.contains(&child_name.as_ref()) {
            write!(f, "    <{}> => {}_el: ", &child_name, &child_name)?;
        } else {
            write!(f, "    <{}>: ", &child_name)?;
        }

        if let Some(t) = type_name {
            writeln!(f, "{},", t)?;
        } else {
            let t = child.get_name().to_upper_camel_case();
            writeln!(f, "{},", t)?;
        }
    }

    writeln!(f, "}});")
}

// define_lists マクロでリストの定義
pub(super) fn write_define_lists<W: Write>(
    f: &mut BufWriter<W>,
    name: &str,
    children: &[SchemaChild],
) -> io::Result<()> {
    writeln!(f, "define_lists!({} {{", name)?;

    for child in children {
        let list_name = child.get_name().to_snake_case();
        let item_name_r = child.schema.children[0].get_name();
        let item_name = item_name_r.to_snake_case();
        let item_struct_name = item_name_r.to_upper_camel_case();
        if RUST_KEYWORDS.contains(&list_name.as_ref()) {
            writeln!(
                f,
                "    <{}> => {}_el: [<{}>: {}],",
                &list_name, &list_name, &item_name, &item_struct_name
            )?;
        } else {
            writeln!(
                f,
                "    <{}>: [<{}>: {}],",
                &list_name, &item_name, &item_struct_name
            )?;
        }
    }

    writeln!(f, "}});")
}

#[expect(unused)]
macro_rules! write_code {
    ($f:expr, $($t:tt)*) => {
        write!($f, "{}", stringify!($($t)*))
    };
}
