use std::{
    char::ParseCharError,
    convert::Infallible,
    net::AddrParseError,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AttrError {
    NotFound(Vec<u8>),
    AddrParseError(AddrParseError),
    ParseBoolError(ParseBoolError),
    ParseCharError(ParseCharError),
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
    EnumParseError(strum::ParseError),
}

impl From<ParseBoolError> for AttrError {
    fn from(value: ParseBoolError) -> Self {
        Self::ParseBoolError(value)
    }
}

impl From<AddrParseError> for AttrError {
    fn from(value: AddrParseError) -> Self {
        Self::AddrParseError(value)
    }
}

impl From<ParseCharError> for AttrError {
    fn from(value: ParseCharError) -> Self {
        Self::ParseCharError(value)
    }
}

impl From<ParseFloatError> for AttrError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl From<ParseIntError> for AttrError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<strum::ParseError> for AttrError {
    fn from(value: strum::ParseError) -> Self {
        Self::EnumParseError(value)
    }
}

impl From<Infallible> for AttrError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
