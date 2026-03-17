use super::write_macros::WriteWithIndent;
use std::{fmt::Display, io, str::FromStr};

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
    },
    EnumU64 {
        name: String,
        variants: Vec<(String, u64)>,
    },
    EnumI32 {
        name: String,
        variants: Vec<(String, i32)>,
    },
    EnumString {
        name: String,
        variants: Vec<(String, String)>,
    },
}

impl ValueType {
    #[allow(dead_code)]
    pub(super) fn new_enum_u32(name: &str, variants: &[(&str, u32)]) -> Self {
        Self::EnumU32 {
            name: name.to_owned(),
            variants: variants
                .iter()
                .map(|(vn, vv)| (vn.to_string(), *vv))
                .collect(),
        }
    }

    #[allow(dead_code)]
    pub(super) fn new_enum_u64(name: &str, variants: &[(&str, u64)]) -> Self {
        Self::EnumU64 {
            name: name.to_owned(),
            variants: variants
                .iter()
                .map(|(vn, vv)| (vn.to_string(), *vv))
                .collect(),
        }
    }

    #[allow(dead_code)]
    pub(super) fn new_enum_i32(name: &str, variants: &[(&str, i32)]) -> Self {
        Self::EnumI32 {
            name: name.to_owned(),
            variants: variants
                .iter()
                .map(|(vn, vv)| (vn.to_string(), *vv))
                .collect(),
        }
    }

    #[allow(dead_code)]
    pub(super) fn new_enum_str(name: &str, variants: &[(&str, &str)]) -> Self {
        Self::EnumString {
            name: name.to_owned(),
            variants: variants
                .iter()
                .map(|(vn, vv)| (vn.to_string(), vv.to_string()))
                .collect(),
        }
    }
}

impl WriteWithIndent for ValueType {
    fn write<W: io::Write>(&self, f: &mut W, indent: &str) -> io::Result<()> {
        match self {
            Self::Primitive(prim) => write!(f, "{}", prim),
            Self::EnumU32 { name, variants } => {
                writeln!(f, "enum {name} u32 {{")?;
                for (vn, vv) in variants {
                    writeln!(f, "{indent}    {vn} = {vv},")?;
                }
                write!(f, "{}}}", indent)
            }
            Self::EnumU64 { name, variants } => {
                writeln!(f, "enum {name} u64 {{")?;
                for (vn, vv) in variants {
                    writeln!(f, "{indent}    {vn} = {vv},")?;
                }
                write!(f, "{}}}", indent)
            }
            Self::EnumI32 { name, variants } => {
                writeln!(f, "enum {name} i32 {{")?;
                for (vn, vv) in variants {
                    writeln!(f, "{indent}    {vn} = {vv},")?;
                }
                write!(f, "{}}}", indent)
            }
            Self::EnumString { name, variants } => {
                writeln!(f, "enum {name} &str {{")?;
                for (vn, vv) in variants {
                    writeln!(f, "{indent}    {vn} = {vv:?},")?;
                }
                write!(f, "{}}}", indent)
            }
        }
    }
}

#[derive(Debug)]
pub(super) enum ChildElementType {
    NamedUnique(&'static str),
    #[expect(dead_code)]
    Unique,
    #[expect(dead_code)]
    List,
}
