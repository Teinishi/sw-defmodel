mod code;
mod node_info;
mod ordered_map;
mod utils;

use code::write_node_code;
use std::{io, path::Path};
use utils::ls_xml;

/*const DEFINE_VEC3I: &str = r#"
define_tag! {
    #[doc = "Represents an element with integer attributes `x`, `y`, and `z`."]
    struct Vec3i {
        "x": i32,
        "y": i32,
        "z": i32,
    }
}
"#;

const DEFINE_VEC3F: &str = r#"
define_tag! {
    #[doc = "Represents an element with float attributes `x`, `y`, and `z`."]
    struct Vec3f {
        "x": f32,
        "y": f32,
        "z": f32,
    }
}
"#;*/

fn main() -> io::Result<()> {
    let test_data_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data");

    // test_data/vanilla_definitions 以下を解析
    let definition = node_info::analyze_files(ls_xml(test_data_path.join("vanilla_definitions"))?);
    //node_info::print_node(&definition, 0);
    write_node_code(
        &mut std::io::stdout(),
        &definition,
        "component definition files",
    )?;

    Ok(())
}

/*// <definition> 用のスキーマの上書きルール
#[derive(Default, Debug)]
struct DefinitionTagRule {
    vec3i: bool,
    vec3f: bool,
}

impl SchemaWriteRule for DefinitionTagRule {
    const MAX_ENUM: usize = 10; // 属性値の自動判定で enum にするしきい値
    const TARGET_LABEL: &str = "component definition files";

    fn before_define_attribute(
        &mut self,
        tag_name: &str,
        attribute: &SchemaAttribute,
    ) -> OverrideAttribute {
        match (tag_name, attribute.get_key().as_ref()) {
            ("definition", "button_type") => OverrideAttribute::enum_u32(
                Some(
                    "A subtype for buttons where the [type attribute][Definition::type_attr()] has a value of 8.",
                ),
                "ButtonType",
                &[
                    ("Push", 0),
                    ("Toggle", 1),
                    ("Key", 2),
                    ("Lockable", 3),
                    ("ThrottleLever", 4),
                    ("SmallKeypad", 5),
                    ("LargeKeypad", 6),
                ],
            ),
            ("definition", "light_type") => {
                OverrideAttribute::enum_u32(None, "LightType", &[("Normal", 0), ("Spotlight", 1)])
            }
            ("surface", "orientation") => OverrideAttribute::enum_u32(
                None,
                "Orientation",
                &[
                    ("XPos", 0),
                    ("XNeg", 1),
                    ("YPos", 2),
                    ("YNeg", 3),
                    ("ZPos", 4),
                    ("ZNeg", 5),
                ],
            ),
            ("surface", "rotation") => OverrideAttribute::enum_u32(
                None,
                "Rotation",
                &[("_0", 0), ("_1", 1), ("_2", 2), ("_3", 3)],
            ),
            ("surface", "shape") | ("surface", "flags") => {
                OverrideAttribute::primitive(None, PrimitiveType::U32)
            }
            ("surface", "trans_type") => {
                OverrideAttribute::enum_u32(None, "TransType", &[("_0", 0), ("_1", 1), ("_2", 2)])
            }
            // TODO: 他
            _ => OverrideAttribute::default(),
        }
    }

    fn before_scan_child(
        &mut self,
        _tag_name: &str,
        child: &SchemaChild,
    ) -> Option<ChildElementType> {
        // x, y, z 属性だけを持ち、整数または浮動小数点数の値は入っているものは Vec3i, Vec3f で定義
        let attrs = &child.schema.attributes;
        if attrs.len() <= 3
            && attrs
                .iter()
                .all(|a| matches!(a.key.as_ref(), "x" | "y" | "z"))
            && child.schema.children.is_empty()
        {
            if attrs.iter().all(|a| parse_ok_all::<i32>(&a.values)) {
                self.vec3i = true;
                return Some(ChildElementType::NamedUnique("Vec3i"));
            } else if attrs.iter().all(|a| parse_ok_all::<f32>(&a.values)) {
                self.vec3f = true;
                return Some(ChildElementType::NamedUnique("Vec3f"));
            }
        }
        None
    }

    fn finalize<W: Write>(
        &mut self,
        f: &mut W,
        tag_name: &str,
        items: &mut Vec<String>,
    ) -> io::Result<()> {
        if tag_name == "definition" {
            if self.vec3i {
                write!(f, "{}", DEFINE_VEC3I)?;
                items.push("Vec3i".to_owned());
            }
            if self.vec3f {
                write!(f, "{}", DEFINE_VEC3F)?;
                items.push("Vec3f".to_owned());
            }
        }

        Ok(())
    }
}*/
