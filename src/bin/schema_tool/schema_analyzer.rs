use super::{
    enums::{ChildElementType, PrimitiveType, ValueType},
    write_macros::{
        write_define_lists, write_define_root, write_define_tag, write_define_unique_children,
    },
    write_rule::SchemaWriteRule,
};
use core::panic;
use heck::{ToSnakeCase, ToUpperCamelCase};
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::{self, File},
    io::{self, BufWriter, Read, Write as _},
    path::{Path, PathBuf},
};
use sw_defmodel::domtree::{Document, Element, HasChildren};

fn get_new_path(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();

    // パスが存在しない場合はそのまま返す
    if !path.exists() {
        return path.to_path_buf();
    }

    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|s| s.to_str());
    let parent = path.parent().unwrap_or_else(|| Path::new(""));

    let mut counter = 1;
    loop {
        // 新しいファイル名を生成
        let new_filename = match ext {
            Some(e) => format!("{}_{}.{}", stem, counter, e),
            None => format!("{}_{}", stem, counter),
        };

        let new_path = parent.join(new_filename);

        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

pub(super) fn analyze_schema<P: AsRef<Path> + Debug, R: SchemaWriteRule>(
    dirs: &[P],
    tag_name: &str,
    module_name: &str,
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

    let output_path = Path::new("tmp").join(module_name);
    let output_path_rs = output_path.with_extension("rs");
    if output_path.is_dir() {
        fs::remove_dir_all(&output_path)?;
    }
    if output_path_rs.is_file() {
        fs::remove_file(&output_path_rs)?;
    }
    fs::create_dir_all(&output_path)?;
    let module_structure = schema.write(&output_path, tag_name, rule)?;

    let mut f = BufWriter::new(File::create(output_path_rs)?);

    let mut stack = vec![&module_structure];
    let mut module_items_map: Vec<(String, Vec<String>)> = Vec::new();
    loop {
        if let Some(m) = stack.pop() {
            if !module_items_map.iter().any(|(k, _)| k == &m.name) {
                writeln!(f, "mod {};", m.name)?;
                let mut items = vec![m.main_item.clone()];
                items.extend_from_slice(&m.items);
                module_items_map.push((m.name.clone(), items));
                for submodule in m.submodules.iter().rev() {
                    stack.push(submodule);
                }
            }
        } else {
            break;
        }
    }
    writeln!(f, "")?;
    for (k, items) in &module_items_map {
        write!(f, "use {k}::{{")?;
        for (i, item) in items.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            f.write_all(item.as_bytes())?;
        }
        writeln!(f, "}};")?;
    }
    writeln!(f, "")?;

    write_define_root(&mut f, tag_name, &module_structure.main_item, rule)?;

    Ok(())
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
                .find(|attr| attr.key.as_bytes() == slot.key())
            {
                attr.update(slot.value())
            } else {
                self.attributes.push(SchemaAttribute::new(
                    std::str::from_utf8(slot.key())
                        .expect("utf-8 error")
                        .to_owned(),
                    slot.value(),
                ));
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
    ) -> io::Result<ModuleStructure> {
        let mut module =
            ModuleStructure::new(tag_name.to_snake_case(), tag_name.to_upper_camel_case());
        let name = &module.name;
        let struct_name = &module.main_item;

        let mut f = Vec::new();

        // 属性定義
        write_define_tag(
            &mut f,
            tag_name,
            &struct_name,
            self.attributes,
            rule,
            &mut module.items,
        )?;

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
            let child_struct_name: String = child_name.to_upper_camel_case();
            writeln!(f, "")?;
            module.items.push(child_struct_name.clone());
            write_define_tag(
                &mut f,
                &child_name,
                &child_struct_name,
                child.schema.attributes,
                rule,
                &mut module.items,
            )?;
        }

        // リストアイテムの write を再帰呼び出し
        for child in child_lists {
            for item in child.schema.children {
                let item_name = item.get_name().to_owned();
                let submodule = item.schema.write(path.as_ref(), &item_name, rule)?;
                module.submodules.push(submodule);
            }
        }

        rule.finalize(&mut f, tag_name, &mut module.items)?;

        // 内容不一致の名前被りがあれば中断
        let path = path.as_ref().join(&name).with_extension("rs");
        if path.is_file() {
            let mut buf = Vec::new();
            File::open(&path)?.read_to_end(&mut buf)?;
            if &buf != &f {
                BufWriter::new(File::create(get_new_path(&path))?).write_all(&f)?;
                panic!("Module name conflict: {}", &name);
            }
        } else {
            BufWriter::new(File::create(path)?).write_all(&f)?;
        }

        Ok(module)
    }
}

#[derive(Debug)]
pub(super) struct SchemaAttribute {
    pub(super) key: String,
    pub(super) values: Vec<String>,
}

impl SchemaAttribute {
    fn new(key: String, value: String) -> Self {
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

    pub(super) fn get_key(&self) -> String {
        self.key.to_snake_case()
    }

    pub(super) fn get_value_type(&self, max_enum: usize) -> ValueType {
        let prim = PrimitiveType::from_values(&self.values);
        if self.values.len() <= max_enum {
            let name = self.key.to_upper_camel_case();
            match prim {
                PrimitiveType::U32 => {
                    return ValueType::EnumU32 {
                        name,
                        variants: self
                            .values
                            .iter()
                            .map(|v| (format!("_{v}"), v.parse().unwrap()))
                            .collect(),
                        doc: None,
                    };
                }
                PrimitiveType::U64 => {
                    return ValueType::EnumU64 {
                        name,
                        variants: self
                            .values
                            .iter()
                            .map(|v| (format!("_{v}"), v.parse().unwrap()))
                            .collect(),
                        doc: None,
                    };
                }
                PrimitiveType::I32 => {
                    return ValueType::EnumI32 {
                        name,
                        variants: self
                            .values
                            .iter()
                            .map(|v| (format!("_{v}"), v.parse().unwrap()))
                            .collect(),
                        doc: None,
                    };
                }
                PrimitiveType::String if self.values.iter().all(|v| !v.contains('/')) => {
                    // スラッシュを含むものはファイルパスとみなして enum 化対象外
                    let mut none_exists = false;
                    let mut variants: Vec<(String, String)> = self
                        .values
                        .iter()
                        .filter_map(|v| {
                            if v.is_empty() {
                                none_exists = true;
                                None
                            } else {
                                Some((v.to_upper_camel_case(), v.clone()))
                            }
                        })
                        .collect();
                    if none_exists {
                        variants.push(("None".to_owned(), String::new()));
                    }
                    return ValueType::EnumString {
                        name,
                        variants,
                        doc: None,
                    };
                }
                _ => {}
            }
        }
        ValueType::Primitive(prim)
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

#[derive(Debug)]
pub(super) struct ModuleStructure {
    pub(super) name: String,
    pub(super) main_item: String,
    pub(super) items: Vec<String>,
    pub(super) submodules: Vec<ModuleStructure>,
}

impl ModuleStructure {
    pub(super) fn new(name: String, main_item: String) -> Self {
        Self {
            name,
            main_item,
            items: Vec::new(),
            submodules: Vec::new(),
        }
    }
}
