use super::{
    MAX_ENUM,
    write_macros::{write_define_lists, write_define_tag, write_define_unique_children},
    write_rule::{ChildElementType, SchemaWriteRule},
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};
use sw_defmodel::domtree::{Document, Element, HasChildren};

pub(super) fn analyze_schema<P: AsRef<Path> + Debug, R: SchemaWriteRule>(
    dirs: &[P],
    tag_name: &str,
    rule: &mut R,
) -> io::Result<()> {
    let mut schema = SchemaElement::default();

    for dir in dirs {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
            {
                let doc = Document::from_file(path).expect("failed to parse {path:?}");
                schema.update(
                    doc.single_element_by_name(tag_name)
                        .expect("failed to get <{tag_name}> in {path:?}")
                        .0,
                );
            }
        }
    }

    let output_dir = Path::new("tmp").join(tag_name);
    fs::create_dir_all(&output_dir)?;

    schema.write(output_dir, tag_name, rule)
}

#[derive(Default, Debug)]
pub(super) struct SchemaElement {
    pub(super) attributes: Vec<SchemaAttribute>,
    pub(super) children: Vec<SchemaChild>,
}

impl SchemaElement {
    fn update(&mut self, element: &Element) {
        for slot in &element.attributes.slots {
            if let Some(attr) = self
                .attributes
                .iter_mut()
                .find(|attr| &attr.key == slot.key())
            {
                attr.update(slot.value())
            } else {
                self.attributes
                    .push(SchemaAttribute::new(slot.key().to_owned(), slot.value()));
            }
        }

        let mut child_count: HashMap<Vec<u8>, usize> = HashMap::new();
        for (child_el, _) in element.elements() {
            if let Some(count) = child_count.get_mut(&child_el.name) {
                *count += 1;
            } else {
                child_count.insert(child_el.name.clone(), 1);
            }

            if let Some(c) = self.children.iter_mut().find(|c| c.name == child_el.name) {
                c.schema.update(child_el);
            } else {
                let mut c = SchemaChild::new(child_el.name.clone());
                c.schema.update(child_el);
                self.children.push(c);
            }
        }

        for c in &mut self.children {
            let count = *child_count.get(&c.name).unwrap_or(&0);
            c.max_count = c.max_count.max(count);
            c.min_count = c.min_count.min(count);
        }
    }

    fn write<P: AsRef<Path>, R: SchemaWriteRule>(
        self,
        path: P,
        tag_name: &str,
        rule: &mut R,
    ) -> io::Result<()> {
        let name = tag_name.to_snake_case();
        let struct_name = tag_name.to_upper_camel_case();

        let mut f = BufWriter::new(File::create(
            path.as_ref().join(&name).with_extension("rs"),
        )?);

        // 属性定義
        write_define_tag(&mut f, tag_name, &struct_name, self.attributes, rule)?;

        // 子要素スキャン
        let mut child_lists = Vec::new();
        let mut unique_children = Vec::new();
        for child in self.children {
            let override_child_type = rule.before_scan_child(tag_name, &child);
            if let Some(ChildElementType::NamedUnique(t)) = override_child_type {
                // 型名上書き
                unique_children.push((child, Some(t)));
            } else if matches!(override_child_type, Some(ChildElementType::List))
                || (child.max_count == 1
                    && child.schema.attributes.is_empty()
                    && child.schema.children.len() == 1)
            {
                // 属性を持たず、単一種類の孫を持っている子要素はリストとみなす
                child_lists.push(child);
            } else if matches!(override_child_type, Some(ChildElementType::Unique))
                || child.max_count == 1
            {
                // 単一子要素
                unique_children.push((child, None));
            } else {
                // 想定外のパターン
                panic!(
                    "Unexpected child element <{}> found in <{}>",
                    child.get_name(),
                    tag_name
                );
            }
        }

        // 単一子要素と親の紐づけを定義
        if !unique_children.is_empty() {
            writeln!(f, "")?;
            write_define_unique_children(&mut f, &struct_name, &unique_children)?;
        }

        // 子リストを定義
        if !child_lists.is_empty() {
            writeln!(f, "")?;
            write_define_lists(&mut f, &struct_name, &child_lists)?;
        }

        // 単一子要素を定義
        for (child, type_name) in unique_children {
            if type_name.is_some() {
                continue;
            }
            let child_name = child.get_name().to_owned();
            let child_struct_name = child_name.to_upper_camel_case();
            writeln!(f, "")?;
            write_define_tag(
                &mut f,
                &child_name,
                &child_struct_name,
                child.schema.attributes,
                rule,
            )?;
        }

        // リストアイテムの write を再帰呼び出し
        for child in child_lists {
            for item in child.schema.children {
                let item_name = item.get_name().to_owned();
                item.schema.write(path.as_ref(), &item_name, rule)?;
            }
        }

        rule.finalize(&mut f, tag_name)
    }
}

#[derive(Debug)]
pub(super) struct SchemaAttribute {
    key: Vec<u8>,
    values: Vec<String>,
}

impl SchemaAttribute {
    fn new(key: Vec<u8>, value: String) -> Self {
        Self {
            key,
            values: Vec::from([value]),
        }
    }

    fn update(&mut self, value: String) {
        if !self.values.contains(&value) {
            self.values.push(value);
        }
    }

    pub(super) fn into_key_type_string(self, indent: &str) -> (String, String) {
        let key = String::from_utf8(self.key).expect("utf-8 error");
        let type_name = key.to_upper_camel_case();
        (
            key.to_snake_case(),
            ValueType::from_values(self.values).as_string(indent, &type_name),
        )
    }
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
    fn from_values(values: &[String]) -> Self {
        if values.iter().all(|v| v.parse::<bool>().is_ok()) {
            Self::Bool
        } else if values.iter().all(|v| v.parse::<u32>().is_ok()) {
            Self::U32
        } else if values.iter().all(|v| v.parse::<u64>().is_ok()) {
            Self::U64
        } else if values.iter().all(|v| v.parse::<i32>().is_ok()) {
            Self::I32
        } else if values.iter().all(|v| v.parse::<f32>().is_ok()) {
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
    Enum(PrimitiveType, Vec<String>),
}

impl ValueType {
    fn from_values(values: Vec<String>) -> Self {
        let prim = PrimitiveType::from_values(&values);
        if values.len() < MAX_ENUM
            && matches!(
                prim,
                PrimitiveType::U32
                    | PrimitiveType::U64
                    | PrimitiveType::I32
                    | PrimitiveType::String
            )
            && values.iter().all(|v| !v.contains('/'))
        // スラッシュを含むものはファイルパスとみなして enum 化対象外
        {
            Self::Enum(prim, values)
        } else {
            Self::Primitive(prim)
        }
    }

    fn as_string(&self, indent: &str, type_name: &str) -> String {
        use std::fmt::Write;

        match self {
            Self::Primitive(prim) => format!("{}", prim),
            Self::Enum(val_type, variants) => {
                let mut f = String::new();

                if matches!(val_type, PrimitiveType::String) {
                    writeln!(&mut f, "enum {} &str {{", type_name).unwrap();
                } else {
                    writeln!(&mut f, "enum {} {} {{", type_name, val_type).unwrap();
                }

                for value in variants {
                    match val_type {
                        PrimitiveType::U32 => {
                            let v = value.parse::<u32>().unwrap();
                            writeln!(&mut f, "{}    _{} = {},", indent, v, v).unwrap();
                        }
                        PrimitiveType::U64 => {
                            let v = value.parse::<u64>().unwrap();
                            writeln!(&mut f, "{}    _{} = {},", indent, v, v).unwrap();
                        }
                        PrimitiveType::I32 => {
                            let v = value.parse::<i32>().unwrap();
                            writeln!(&mut f, "{}    _{} = {},", indent, v, v).unwrap();
                        }
                        PrimitiveType::String => {
                            if value.is_empty() {
                                writeln!(&mut f, "{}    None = {:?},", indent, value).unwrap();
                            } else {
                                writeln!(
                                    &mut f,
                                    "{}    {} = {:?},",
                                    indent,
                                    value.to_upper_camel_case(),
                                    value
                                )
                                .unwrap();
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                write!(&mut f, "{}}}", indent).unwrap();

                f
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct SchemaChild {
    pub(super) name: Vec<u8>,
    pub(super) min_count: usize,
    pub(super) max_count: usize,
    pub(super) schema: SchemaElement,
}

impl SchemaChild {
    fn new(name: Vec<u8>) -> Self {
        Self {
            name,
            min_count: usize::MAX,
            max_count: 0,
            schema: SchemaElement::default(),
        }
    }

    pub(super) fn get_name(&self) -> &str {
        std::str::from_utf8(&self.name).expect("utf-8 error")
    }
}
