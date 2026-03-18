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
    #[allow(dead_code)]
    pub(super) fn new_enum_u32(
        name: &str,
        variants: &[(&str, u32)],
        doc: Option<&'static str>,
    ) -> Self {
        Self::EnumU32 {
            name: name.to_owned(),
            variants: variants
                .iter()
                .map(|(vn, vv)| (vn.to_string(), *vv))
                .collect(),
            doc,
        }
    }

    #[allow(dead_code)]
    pub(super) fn new_enum_u64(
        name: &str,
        variants: &[(&str, u64)],
        doc: Option<&'static str>,
    ) -> Self {
        Self::EnumU64 {
            name: name.to_owned(),
            variants: variants
                .iter()
                .map(|(vn, vv)| (vn.to_string(), *vv))
                .collect(),
            doc,
        }
    }

    #[allow(dead_code)]
    pub(super) fn new_enum_i32(
        name: &str,
        variants: &[(&str, i32)],
        doc: Option<&'static str>,
    ) -> Self {
        Self::EnumI32 {
            name: name.to_owned(),
            variants: variants
                .iter()
                .map(|(vn, vv)| (vn.to_string(), *vv))
                .collect(),
            doc,
        }
    }

    #[allow(dead_code)]
    pub(super) fn new_enum_str(
        name: &str,
        variants: &[(&str, &str)],
        doc: Option<&'static str>,
    ) -> Self {
        Self::EnumString {
            name: name.to_owned(),
            variants: variants
                .iter()
                .map(|(vn, vv)| (vn.to_string(), vv.to_string()))
                .collect(),
            doc,
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
            } => write_enum_with_idnent(f, name, "u32", doc, variants, indent),
            Self::EnumU64 {
                name,
                variants,
                doc,
            } => write_enum_with_idnent(f, name, "u64", doc, variants, indent),
            Self::EnumI32 {
                name,
                variants,
                doc,
            } => write_enum_with_idnent(f, name, "i32", doc, variants, indent),
            Self::EnumString {
                name,
                variants,
                doc,
            } => write_enum_with_idnent(f, name, "&str", doc, variants, indent),
        }
    }
}

fn write_enum_with_idnent<W: io::Write, K: Display, T: Debug>(
    f: &mut W,
    name: &str,
    val_type: &str,
    doc: &Option<&str>,
    variants: &[(K, T)],
    indent: &str,
) -> io::Result<()> {
    if let Some(doc) = doc {
        write!(f, "#[doc = {:?}] ", doc)?;
    }
    write!(f, "enum {name} {val_type} ")?;
    writeln!(f, "{{")?;

    for (vn, vv) in variants {
        writeln!(f, "{indent}    {vn} = {vv:?},")?;
    }
    write!(f, "{indent}}}")
}

#[derive(Debug)]
pub(super) enum ChildElementType {
    NamedUnique(&'static str),
    #[expect(dead_code)]
    Unique,
    #[expect(dead_code)]
    List,
}
