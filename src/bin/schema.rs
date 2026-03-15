// サンプルXMLを読んでスキーマ生成
use std::{collections::HashMap, fmt::Display, path::Path};
use sw_defmodel::domtree::{Document, Element, HasChildren};
const MAX_ENUM: usize = 10;

#[derive(Default, Debug)]
struct Schema {
    attributes: Vec<SchemaAttribute>,
    children: Vec<SchemaChild>,
}

impl Schema {
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

    fn finalize(self, prefix: Option<&str>) {
        for attr in self.attributes {
            if let Some(prefix) = prefix {
                print!("{prefix}/");
            }
            print!(
                "@{}: ",
                std::str::from_utf8(&attr.key).expect("invalid utf-8")
            );
            println!("{}", attr.get_type());
        }

        for child in &self.children {
            assert!(child.min_count <= child.max_count);
            if let Some(prefix) = prefix {
                print!("{prefix}/");
            }
            println!(
                "{}: {}",
                std::str::from_utf8(&child.name).expect("invalid utf-8"),
                child.get_type()
            );
        }

        for child in self.children {
            let child_name = std::str::from_utf8(&child.name).expect("invalid utf-8");
            if let Some(prefix) = prefix {
                child
                    .schema
                    .finalize(Some(&format!("{prefix}/{child_name}")));
            } else {
                child.schema.finalize(Some(child_name));
            }
        }
    }
}

#[derive(Debug)]
struct SchemaAttribute {
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

    fn get_type(self) -> ValueType {
        if self.values.iter().all(|v| v.parse::<bool>().is_ok()) {
            ValueType::Bool
        } else if self.values.iter().all(|v| v.parse::<u32>().is_ok()) {
            if self.values.len() <= MAX_ENUM {
                ValueType::Enum(self.values)
            } else {
                ValueType::U32
            }
        } else if self.values.iter().all(|v| v.parse::<i32>().is_ok()) {
            ValueType::I32
        } else if self.values.iter().all(|v| v.parse::<f32>().is_ok()) {
            ValueType::F32
        } else if self.values.len() <= MAX_ENUM {
            ValueType::Enum(self.values)
        } else {
            ValueType::String
        }
    }
}

#[derive(Debug)]
enum ValueType {
    Bool,
    U32,
    I32,
    F32,
    Enum(Vec<String>),
    String,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::U32 => write!(f, "u32"),
            Self::I32 => write!(f, "i32"),
            Self::F32 => write!(f, "f32"),
            Self::Enum(values) => {
                if let Some((last, rest)) = values.split_last() {
                    for v in rest {
                        write!(f, "{v:?} | ")?;
                    }
                    write!(f, "{last:?}")
                } else {
                    write!(f, "None")
                }
            }
            Self::String => write!(f, "String"),
        }
    }
}

#[derive(Debug)]
struct SchemaChild {
    name: Vec<u8>,
    min_count: usize,
    max_count: usize,
    schema: Schema,
}

impl SchemaChild {
    fn new(name: Vec<u8>) -> Self {
        Self {
            name,
            min_count: usize::MAX,
            max_count: 0,
            schema: Schema::default(),
        }
    }

    fn get_type(&self) -> String {
        if self.min_count == 1 && self.max_count == 1 {
            "Always".to_owned()
        } else if self.min_count == 0 && self.max_count == 1 {
            "Optional".to_owned()
        } else {
            "Vec".to_owned()
        }
    }
}

fn main() {
    let definitions_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("vanilla_definitions");

    let mut schema = Schema::default();

    for entry in std::fs::read_dir(&definitions_dir)
        .unwrap_or_else(|e| panic!("failed to read dir {definitions_dir:?}: {e}"))
    {
        let entry =
            entry.unwrap_or_else(|e| panic!("failed to read entry in {definitions_dir:?}: {e}"));
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
        {
            let doc = Document::from_file(path).expect("failed to read {path:?}");
            schema.update(
                doc.single_element_by_name("definition")
                    .expect("failed to get <definition> in {path:?}")
                    .0,
            );
        }
    }

    schema.finalize(None);
}
