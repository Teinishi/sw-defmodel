// サンプルXMLを読んでスキーマ生成
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};
use sw_defmodel::domtree::{Document, Element, HasChildren};

const MAX_ENUM: usize = 10;

const RUST_KEYWORDS: [&str; 50] = [
    "as", "async", "await", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

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

    fn write<W: Write>(self, f: &mut BufWriter<W>, tag_name: &str) -> io::Result<()> {
        use heck::ToUpperCamelCase;
        let struct_name = tag_name.to_upper_camel_case();

        writeln!(f, "define_attributes! {{")?;
        writeln!(f, "    {tag_name:?} => {struct_name} {{")?;

        for attr in self.attributes {
            let key = attr.get_key();
            if RUST_KEYWORDS.contains(&key) {
                write!(f, "        {:?} => {}_attr: ", key, key)?;
            } else {
                write!(f, "        {:?}: ", key)?;
            }
            let ident = std::str::from_utf8(&attr.key)
                .expect("utf-8 error")
                .to_upper_camel_case();
            attr.get_type().fmt(f, "        ", &ident)?;
            writeln!(f, ",")?;
        }

        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;

        Ok(())
    }

    /*fn finalize(self, prefix: Option<&str>) {
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
    }*/
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

    fn get_key(&self) -> &str {
        std::str::from_utf8(&self.key).expect("utf-8 error")
    }

    fn get_type(self) -> ValueType {
        ValueType::from_values(self.values)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PrimitiveType {
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
enum ValueType {
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
        {
            Self::Enum(prim, values)
        } else {
            Self::Primitive(prim)
        }
    }

    fn fmt<W: Write>(&self, f: &mut BufWriter<W>, indent: &str, type_name: &str) -> io::Result<()> {
        use heck::ToUpperCamelCase;

        match self {
            Self::Primitive(prim) => write!(f, "{}", prim),
            Self::Enum(val_type, variants) => {
                if matches!(val_type, PrimitiveType::String) {
                    writeln!(f, "enum {} &str {{", type_name)?;
                } else {
                    writeln!(f, "enum {} {} {{", type_name, val_type)?;
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

fn generate_schema<P: AsRef<Path> + Debug>(dir: P, tag_name: &str) -> io::Result<()> {
    let mut schema = Schema::default();

    for entry in std::fs::read_dir(&dir)? {
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

    let output_path = Path::new("tmp").join(tag_name).with_extension("rs");

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    /*if output_path.exists() {
        print!(
            "File '{}' already exists. Overwrite? (y/n): ",
            output_path.display()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("Canceled.");
            return Ok(());
        }
    }*/

    let file = File::create(&output_path)?;
    schema.write(&mut BufWriter::new(file), tag_name)
}

fn main() {
    generate_schema(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("vanilla_definitions"),
        "definition",
    )
    .unwrap();
}
