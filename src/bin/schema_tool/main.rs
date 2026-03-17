mod primitive_type;
mod schema_analyzer;
mod write_macros;
mod write_rule;

use primitive_type::parse_ok_all;
use schema_analyzer::{SchemaChild, analyze_schema};
use std::{
    io::{self, Write},
    path::Path,
};
use write_rule::{ChildElementType, SchemaWriteRule};

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
        }

        Ok(())
    }
}
