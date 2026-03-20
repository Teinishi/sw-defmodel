use super::write_macros::WriteWithIndent;
use std::{
    fmt::{Debug, Display},
    io,
    str::FromStr,
};

pub(super) fn parse_ok_all<T: FromStr>(values: &[String]) -> bool {
    values.iter().all(|v| v.parse::<T>().is_ok())
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(super) enum PrimitiveType {
    Bool,
    U32,
    U64,
    I32,
    F32,
    String,
}

impl PrimitiveType {
    pub(super) fn from_values(values: &[String]) -> Self {
        if parse_ok_all::<bool>(values) {
            Self::Bool
        } else if parse_ok_all::<u32>(values) {
            Self::U32
        } else if parse_ok_all::<u64>(values) {
            Self::U64
        } else if parse_ok_all::<i32>(values) {
            Self::I32
        } else if parse_ok_all::<f32>(values) {
            Self::F32
        } else {
            Self::String
        }
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::I32 => write!(f, "i32"),
            Self::F32 => write!(f, "f32"),
            Self::String => write!(f, "String"),
        }
    }
}

#[derive(Debug)]
pub(super) enum ValueType {
    Primitive(PrimitiveType),
    EnumU32 {
        name: String,
        variants: Vec<(String, u32)>,
        doc: Option<&'static str>,
    },
    EnumU64 {
        name: String,
        variants: Vec<(String, u64)>,
        doc: Option<&'static str>,
    },
    EnumI32 {
        name: String,
        variants: Vec<(String, i32)>,
        doc: Option<&'static str>,
    },
    EnumString {
        name: String,
        variants: Vec<(String, String)>,
        doc: Option<&'static str>,
    },
}

impl ValueType {
    pub(super) fn get_enum_name(&self) -> Option<&String> {
        match self {
            Self::Primitive(_) => None,
            Self::EnumU32 { name, .. }
            | Self::EnumU64 { name, .. }
            | Self::EnumI32 { name, .. }
            | Self::EnumString { name, .. } => Some(name),
        }
    }
}

impl WriteWithIndent for ValueType {
    fn write<W: io::Write>(&self, f: &mut W, indent: &str) -> io::Result<()> {
        match self {
            Self::Primitive(prim) => write!(f, "{}", prim),
            Self::EnumU32 {
                name,
                variants,
                doc,
            } => write_enum_with_idnent(f, doc, name, "u32", variants, indent),
            Self::EnumU64 {
                name,
                variants,
                doc,
            } => write_enum_with_idnent(f, doc, name, "u64", variants, indent),
            Self::EnumI32 {
                name,
                variants,
                doc,
            } => write_enum_with_idnent(f, doc, name, "i32", variants, indent),
            Self::EnumString {
                name,
                variants,
                doc,
            } => write_enum_with_idnent(f, doc, name, "&str", variants, indent),
        }
    }
}

fn write_enum_with_idnent<W: io::Write, K: Display, T: Debug>(
    f: &mut W,
    doc: &Option<&str>,
    name: &str,
    val_type: &str,
    variants: &[(K, T)],
    indent: &str,
) -> io::Result<()> {
    let indent1 = if let Some(doc) = doc {
        writeln!(f, "\n{indent}    #[doc = {doc:?}] ")?;
        writeln!(f, "{indent}    enum {name} {val_type} {{")?;
        "    "
    } else {
        writeln!(f, "enum {name} {val_type} {{")?;
        ""
    };

    for (vn, vv) in variants {
        writeln!(f, "{indent}{indent1}    {vn} = {vv:?},")?;
    }
    write!(f, "{indent}{indent1}}}")
}

#[derive(Debug)]
pub(super) enum ChildElementType {
    NamedUnique(&'static str),
    #[expect(dead_code)]
    Unique,
    #[expect(dead_code)]
    List,
}
