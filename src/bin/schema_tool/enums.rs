use super::write_macros::WriteWithIndent;
use heck::ToUpperCamelCase;
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
    Enum(String, PrimitiveType, Vec<String>),
}

impl WriteWithIndent for ValueType {
    fn write<W: io::Write>(&self, f: &mut W, indent: &str) -> io::Result<()> {
        match self {
            Self::Primitive(prim) => write!(f, "{}", prim),
            Self::Enum(name, val_type, variants) => {
                if matches!(val_type, PrimitiveType::String) {
                    writeln!(f, "enum {} &str {{", name)?;
                } else {
                    writeln!(f, "enum {} {} {{", name, val_type)?;
                }

                for value in variants {
                    match val_type {
                        PrimitiveType::U32 => {
                            let v = value.parse::<u32>().unwrap();
                            writeln!(f, "{}    _{} = {},", indent, v, v)?;
                        }
                        PrimitiveType::U64 => {
                            let v = value.parse::<u64>().unwrap();
                            writeln!(f, "{}    _{} = {},", indent, v, v)?;
                        }
                        PrimitiveType::I32 => {
                            let v = value.parse::<i32>().unwrap();
                            writeln!(f, "{}    _{} = {},", indent, v, v)?;
                        }
                        PrimitiveType::String => {
                            if value.is_empty() {
                                writeln!(f, "{}    None = {:?},", indent, value)?;
                            } else {
                                writeln!(
                                    f,
                                    "{}    {} = {:?},",
                                    indent,
                                    value.to_upper_camel_case(),
                                    value
                                )?;
                            }
                        }
                        _ => unreachable!(),
                    }
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
