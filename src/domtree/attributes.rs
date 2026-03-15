use super::{
    error::AttrError,
    utils::{debug_utf8, escape_xml, leading_whitespaces, unescape_xml},
};
use std::{
    fmt::{Debug, Display},
    io::Write,
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Attributes {
    pub slots: Vec<AttrSlot>,
    pub trailing_space: Vec<u8>,
}

impl Debug for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[expect(dead_code)]
        #[derive(Debug)]
        struct Attributes<'a> {
            slots: &'a Vec<AttrSlot>,
            trailing_space: &'a str,
        }
        let tmp = Attributes {
            slots: &self.slots,
            trailing_space: debug_utf8(&self.trailing_space),
        };
        tmp.fmt(f)
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            trailing_space: b" ".to_vec(),
        }
    }
}

impl Attributes {
    pub fn new(input: &[u8]) -> Self {
        let (slots, trailing_space) = AttrScanner::new(input).scan();
        Self {
            slots,
            trailing_space,
        }
    }

    pub fn get_unescaped<K: AsRef<[u8]>>(&self, key: K) -> Option<String> {
        let key = key.as_ref();
        self.slots
            .iter()
            .find(|slot| slot.key == key)
            .map(|slot| slot.get_unescaped())
    }

    pub fn set_unescaped<K: AsRef<[u8]>>(&mut self, key: K, value: &str) {
        let key = key.as_ref();
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.key == key) {
            slot.set_unescaped(value.as_bytes());
            return;
        }

        // 末尾に新規追加する場合、直前の属性を真似する
        let leading = self
            .slots
            .last()
            .map(|a| leading_whitespaces(&a.prefix))
            .unwrap_or(b" ");

        let (escaped_value, quote) = escape_xml(value.as_bytes());
        let mut prefix = Vec::with_capacity(leading.len() + key.len() + 2);
        prefix.extend_from_slice(leading);
        prefix.extend_from_slice(key);
        prefix.extend_from_slice(b"=");

        self.slots.push(AttrSlot {
            prefix,
            key: key.to_vec(),
            quote,
            value: escaped_value,
        });
    }

    pub fn get<K: AsRef<[u8]>, T, E>(&self, key: K) -> Result<T, AttrError>
    where
        T: FromStr<Err = E>,
        AttrError: From<E>,
    {
        let key = key.as_ref();
        if let Some(r) = self
            .slots
            .iter()
            .find(|slot| slot.key == key)
            .map(|slot| T::from_str(&unescape_xml(&slot.value)))
        {
            Ok(r?)
        } else {
            Err(AttrError::NotFound(key.as_ref().into()))
        }
    }

    pub fn set<K: AsRef<[u8]>, T: Display>(&mut self, key: K, value: T) {
        self.set_unescaped(key, &value.to_string());
    }

    pub fn remove<K: AsRef<[u8]>>(&mut self, key: K) -> Option<AttrSlot> {
        let key = key.as_ref();
        let i = self
            .slots
            .iter()
            .enumerate()
            .find_map(|(i, slot)| (slot.key == key).then_some(i));
        if let Some(i) = i {
            Some(self.slots.remove(i))
        } else {
            None
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for attr in &self.slots {
            writer.write_all(&attr.prefix)?;
            writer.write_all(&[attr.quote])?;
            writer.write_all(&attr.value)?;
            writer.write_all(&[attr.quote])?;
        }
        writer.write_all(&self.trailing_space)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AttrSlot {
    prefix: Vec<u8>,
    key: Vec<u8>,
    quote: u8,
    value: Vec<u8>,
}

impl Debug for AttrSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[expect(dead_code)]
        #[derive(Debug)]
        struct AttrSlot<'a> {
            prefix: &'a str,
            key: &'a str,
            quote: char,
            value: &'a str,
        }
        let tmp = AttrSlot {
            prefix: debug_utf8(&self.prefix),
            key: debug_utf8(&self.key),
            quote: self.quote.into(),
            value: debug_utf8(&self.value),
        };
        tmp.fmt(f)
    }
}

impl AttrSlot {
    pub fn key(&self) -> &Vec<u8> {
        &self.key
    }

    pub fn get_unescaped(&self) -> String {
        unescape_xml(&self.value)
    }
    pub fn set_unescaped(&mut self, value: &[u8]) {
        let (escaped_value, quote) = escape_xml(value);
        self.value = escaped_value;
        self.quote = quote;
    }
}

#[derive(Debug)]
struct AttrScanner<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> AttrScanner<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).cloned()
    }

    fn consume(&mut self) -> Option<u8> {
        let b = self.peek();
        if b.is_some() {
            self.pos += 1;
        }
        b
    }

    fn consume_whitespace(&mut self) -> Vec<u8> {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if b.is_ascii_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_vec()
    }

    fn scan(&mut self) -> (Vec<AttrSlot>, Vec<u8>) {
        let mut slots = Vec::new();
        let trailing_space;

        loop {
            let start_pos = self.pos;
            let leading = self.consume_whitespace();

            // 属性名が始まらないまま末尾に達したなら、それは末尾の空白
            let name_start = self.pos;
            if self.peek().is_none() {
                trailing_space = leading;
                break;
            }

            // 属性名を取得
            while let Some(b) = self.peek() {
                if b == b'=' || b.is_ascii_whitespace() {
                    break;
                }
                self.consume();
            }
            let key = self.input[name_start..self.pos].to_vec();

            self.consume_whitespace();
            if self.consume() != Some(b'=') {
                // 文法エラー
                todo!();
            }
            self.consume_whitespace();

            let quote = self.consume().unwrap_or(b'"');

            if quote != b'"' && quote != b'\'' {
                // TODO: Err を返すようにする
                panic!("Unexpected quote: {}", quote);
            }

            // 属性値のパース
            let value_start = self.pos;
            while self.consume().is_some_and(|b| b != quote) {}
            let value_end_with_quote = self.pos;

            let prefix: Vec<u8> = self.input[start_pos..value_start - 1].to_vec();
            let value = self.input[value_start..value_end_with_quote - 1].to_vec();
            slots.push(AttrSlot {
                prefix,
                key,
                quote,
                value,
            });
        }

        (slots, trailing_space)
    }
}
