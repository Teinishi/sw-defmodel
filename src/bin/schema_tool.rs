// サンプルXMLを読んでスキーマ生成
use heck::{ToSnakeCase, ToUpperCamelCase};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};
use sw_defmodel::domtree::{Document, Element, HasChildren};

const MAX_ENUM: usize = 10;

fn main() {
    // test_data/vanilla_definitions から <definition> のスキーマを生成
    generate_schema(
        &[Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("vanilla_definitions")],
        "definition",
        &mut DefinitionTagRule::default(),
    )
    .unwrap();
}

// <definition> のスキーマの上書きルール
#[derive(Default, Debug)]
struct DefinitionTagRule {
    vec3i: bool,
    vec3f: bool,
}

impl SchemaWriteRule for DefinitionTagRule {
    fn before_scan_child(
        &mut self,
        tag_name: &str,
        child: &SchemaChild,
    ) -> Option<ChildElementType> {
        if tag_name == "definition" {
            match child.get_name() {
                "voxel_min"
                | "voxel_max"
                | "voxel_physics_min"
                | "voxel_physics_max"
                | "voxel_location_child"
                | "light_position"
                | "dynamic_body_position"
                | "compartment_sample_pos"
                | "seat_front"
                | "seat_up"
                | "light_forward"
                | "door_normal"
                | "door_side"
                | "door_up"
                | "door_base_pos"
                | "connector_axis"
                | "connector_up"
                | "particle_direction"
                | "seat_exit_position"
                | "weapon_breech_position"
                | "weapon_breech_normal" => {
                    self.vec3i = true;
                    Some(ChildElementType::NamedUnique("Vec3i"))
                }
                "bb_physics_min"
                | "bb_physics_max"
                | "constraint_pos_parent"
                | "constraint_pos_child"
                | "force_dir"
                | "light_color"
                | "door_size"
                | "dynamic_rotation_axes"
                | "dynamic_side_axis"
                | "magnet_offset"
                | "seat_offset"
                | "seat_camera"
                | "seat_render"
                | "particle_offset"
                | "particle_bounds"
                | "rope_hook_offset"
                | "weapon_cart_position"
                | "weapon_cart_velocity" => {
                    self.vec3f = true;
                    Some(ChildElementType::NamedUnique("Vec3f"))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn finalize<W: Write>(&mut self, f: &mut BufWriter<W>, tag_name: &str) -> io::Result<()> {
        if self.vec3i {
            writeln!(f, "")?;
            writeln!(f, "define_tag!(Vec3i {{")?;
            writeln!(f, "    \"x\": i32,")?;
            writeln!(f, "    \"y\": i32,")?;
            writeln!(f, "    \"z\": i32,")?;
            writeln!(f, "}});")?;
        }
        if self.vec3f {
            writeln!(f, "")?;
            writeln!(f, "define_tag!(Vec3f {{")?;
            writeln!(f, "    \"x\": f32,")?;
            writeln!(f, "    \"y\": f32,")?;
            writeln!(f, "    \"z\": f32,")?;
            writeln!(f, "}});")?;
        }

        Ok(())
    }
}

trait SchemaWriteRule {
    #[expect(unused_variables)]
    fn before_define_attribute(
        &mut self,
        tag_name: &str,
        attribute: &SchemaAttribute,
    ) -> Option<String> {
        None
    }

    #[expect(unused_variables)]
    fn before_scan_child(
        &mut self,
        tag_name: &str,
        child: &SchemaChild,
    ) -> Option<ChildElementType> {
        None
    }

    #[expect(unused_variables)]
    fn finalize<W: Write>(&mut self, f: &mut BufWriter<W>, tag_name: &str) -> io::Result<()> {
        Ok(())
    }
}

fn generate_schema<P: AsRef<Path> + Debug, R: SchemaWriteRule>(
    dirs: &[P],
    tag_name: &str,
    rule: &mut R,
) -> io::Result<()> {
    let mut schema = Schema::default();

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
    // rule.finalize()
}

const RUST_KEYWORDS: [&str; 50] = [
    "as", "async", "await", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

// define_tag マクロで属性定義
fn write_define_tag<W: Write, R: SchemaWriteRule>(
    f: &mut BufWriter<W>,
    tag_name: &str,
    name: &str,
    attributes: Vec<SchemaAttribute>,
    rule: &mut R,
) -> io::Result<()> {
    writeln!(f, "define_tag!({name} {{")?;

    for attr in attributes {
        let override_typename = rule.before_define_attribute(tag_name, &attr);
        let (mut key, val_type) = attr.into_key_type_string("    ");
        if let Some(n) = override_typename {
            key = n;
        }

        if RUST_KEYWORDS.contains(&key.as_str()) {
            writeln!(f, "    {:?} => {}_attr: {},", key, key, val_type)?;
        } else {
            writeln!(f, "    {:?}: {},", key, val_type)?;
        }
    }

    writeln!(f, "}});")
}

// define_unique_children マクロで親と子要素の紐づけ定義
fn write_define_unique_children<W: Write>(
    f: &mut BufWriter<W>,
    name: &str,
    children: &[(SchemaChild, Option<&'static str>)],
) -> io::Result<()> {
    writeln!(f, "define_unique_children!({} {{", name)?;

    for (child, type_name) in children {
        let child_name = child.get_name().to_snake_case();
        if RUST_KEYWORDS.contains(&child_name.as_ref()) {
            write!(f, "    <{}> => {}_el: ", &child_name, &child_name)?;
        } else {
            write!(f, "    <{}>: ", &child_name)?;
        }

        if let Some(t) = type_name {
            writeln!(f, "{},", t)?;
        } else {
            let t = child.get_name().to_upper_camel_case();
            writeln!(f, "{},", t)?;
        }
    }

    writeln!(f, "}});")
}

// define_lists マクロでリストの定義
fn write_define_lists<W: Write>(
    f: &mut BufWriter<W>,
    name: &str,
    children: &[SchemaChild],
) -> io::Result<()> {
    writeln!(f, "define_lists!({} {{", name)?;

    for child in children {
        let list_name = child.get_name().to_snake_case();
        let item_name_r = child.schema.children[0].get_name();
        let item_name = item_name_r.to_snake_case();
        let item_struct_name = item_name_r.to_upper_camel_case();
        if RUST_KEYWORDS.contains(&list_name.as_ref()) {
            writeln!(
                f,
                "    <{}> => {}_el: [<{}>: {}],",
                &list_name, &list_name, &item_name, &item_struct_name
            )?;
        } else {
            writeln!(
                f,
                "    <{}>: [<{}>: {}],",
                &list_name, &item_name, &item_struct_name
            )?;
        }
    }

    writeln!(f, "}});")
}

#[derive(Debug)]
enum ChildElementType {
    NamedUnique(&'static str),
    #[expect(dead_code)]
    Unique,
    #[expect(dead_code)]
    List,
}

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

    fn into_key_type_string(self, indent: &str) -> (String, String) {
        let key = String::from_utf8(self.key).expect("utf-8 error");
        let type_name = key.to_upper_camel_case();
        (
            key.to_snake_case(),
            ValueType::from_values(self.values).as_string(indent, &type_name),
        )
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

    fn get_name(&self) -> &str {
        std::str::from_utf8(&self.name).expect("utf-8 error")
    }
}
