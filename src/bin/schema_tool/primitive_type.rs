use std::{fmt::Display, str::FromStr};

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
