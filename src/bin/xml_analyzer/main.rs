mod code;
mod code_rule;
mod node_info;
mod ordered_map;
mod utils;

use code::write_code;
use code_rule::{ChildClassificcation, CodeRule, NamePath};
use node_info::{ChildInfo, ValueType};
use std::{
    fs::{self, File},
    io::{self, BufWriter},
    path::Path,
};
use utils::ls_xml;

const DEFINE_VEC3I: &str = r#"define_tag! {
    #[doc = "Represents an element with integer attributes `x`, `y`, and `z`."]
    struct Vec3i {
        "x": i32,
        "y": i32,
        "z": i32,
    }
}

"#;

const DEFINE_VEC3F: &str = r#"define_tag! {
    #[doc = "Represents an element with float attributes `x`, `y`, and `z`."]
    struct Vec3f {
        "x": f32,
        "y": f32,
        "z": f32,
    }
}

"#;

fn main() -> io::Result<()> {
    let test_data_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data");
    let output_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tmp");

    // test_data/vanilla_definitions 以下を解析
    let definition = node_info::analyze_files(ls_xml(test_data_path.join("vanilla_definitions"))?);
    fs::create_dir_all(&output_path)?;
    let mut f = BufWriter::new(File::create(output_path.join("component_definition.rs"))?);
    write_code(&mut f, &definition, &mut DefinitionRule::default())?;

    Ok(())
}

// <definition> 用の上書きルール
#[derive(Default, Debug)]
struct DefinitionRule {
    vec3i: bool,
    vec3f: bool,
}

impl CodeRule for DefinitionRule {
    const TARGET_LABEL: &str = "component definition files";

    fn override_child(
        &mut self,
        path: &NamePath,
        info: &ChildInfo,
    ) -> Option<ChildClassificcation> {
        match path.name() {
            "jet_engine_connections_prev" => {
                return Some(ChildClassificcation::list());
            }
            "particle_offset" | "particle_bounds" => {
                return Some(ChildClassificcation::unique());
            }
            _ => {}
        }

        let attrs = &info.inner().attributes;
        let n = attrs.len();
        if (1..=3).contains(&n)
            && attrs
                .iter()
                .all(|(name, _)| matches!(name.as_str(), "x" | "y" | "z"))
        {
            if attrs
                .iter()
                .all(|(_, a)| matches!(a.types.last(), Some(ValueType::F32)))
            {
                self.vec3i = true;
                return Some(ChildClassificcation::unique_inline("Vec3f"));
            } else if attrs
                .iter()
                .all(|(_, a)| matches!(a.types.last(), Some(ValueType::I32 | ValueType::U32)))
            {
                self.vec3f = true;
                return Some(ChildClassificcation::unique_inline("Vec3i"));
            }
        }

        None
    }

    fn finalize<W: io::Write>(&mut self, f: &mut W) -> io::Result<()> {
        if self.vec3i {
            write!(f, "{}", DEFINE_VEC3I)?;
        }
        if self.vec3f {
            write!(f, "{}", DEFINE_VEC3F)?;
        }

        Ok(())
    }
}
