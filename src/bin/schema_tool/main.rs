#[macro_use]
mod write_macros;

mod enums;
mod schema_analyzer;
mod write_rule;

use enums::{ChildElementType, parse_ok_all};
use schema_analyzer::{SchemaAttribute, SchemaChild, analyze_schema};
use std::{
    io::{self, Write},
    path::Path,
};
use write_rule::{OverrideAttribute, SchemaWriteRule};

const DEFINE_VEC3I: &str = r#"
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
"#;

fn main() -> io::Result<()> {
    // test_data/vanilla_definitions から <definition> のスキーマを生成
    analyze_schema(
        &[Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("vanilla_definitions")],
        "definition",
        &mut DefinitionTagRule::default(),
    )?;

    Ok(())
}

// <definition> 用のスキーマの上書きルール
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
        if tag_name == "definition" {
            match attribute.get_key().as_ref() {
                "button_type" => {
                    return OverrideAttribute::enum_u32(
                        "A subtype for buttons where the [type attribute][Definition::type_attr()] has a value of 8.",
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
                    );
                }
                "light_type" => {
                    return OverrideAttribute::enum_u32(
                        "", // TODO
                        "LightType",
                        &[("Normal", 0), ("Spotlight", 1)],
                    );
                }
                // TODO: 他
                _ => {}
            }
        }
        Default::default()
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

    fn finalize<W: Write>(&mut self, f: &mut io::BufWriter<W>, tag_name: &str) -> io::Result<()> {
        if tag_name == "definition" {
            if self.vec3i {
                write!(f, "{}", DEFINE_VEC3I)?;
            }
            if self.vec3f {
                write!(f, "{}", DEFINE_VEC3F)?;
            }
        }

        Ok(())
    }
}
