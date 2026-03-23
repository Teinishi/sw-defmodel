mod code;
mod code_rule;
mod node_info;
mod ordered_map;
mod utils;

use code::write_code;
use code_rule::{ChildClassificcation, CodeRule, NamePath};
use node_info::{ChildInfo, ValueType, analyze_files};
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
    // test_data/vanilla_definitions 以下を解析
    analyze_and_write(
        "vanilla_definitions",
        "definition",
        DefinitionRule::default(),
    )?;

    // test_data/vehicles 以下を解析
    analyze_and_write("vehicles", "vehicle", VehicleRule)?;

    Ok(())
}

fn analyze_and_write<R: CodeRule>(
    test_data_subdir: &str,
    module_name: &str,
    mut rule: R,
) -> io::Result<()> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let xml_path = manifest_path.join("test_data").join(test_data_subdir);

    println!("Analyzing files in {xml_path:?}");

    let info = analyze_files(ls_xml(xml_path)?);

    let output_path = manifest_path
        .join("tmp")
        .join(module_name)
        .with_extension("rs");
    println!("Writing to {output_path:?}");

    fs::create_dir_all(output_path.parent().unwrap())?;
    let mut f = BufWriter::new(File::create(output_path)?);
    write_code(&mut f, &info, &mut rule)?;

    println!("Done");
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
            if attrs.iter().all(|(_, a)| matches!(a.ty(), ValueType::F32)) {
                self.vec3i = true;
                return Some(ChildClassificcation::unique_inline("Vec3f"));
            } else if attrs
                .iter()
                .all(|(_, a)| matches!(a.ty(), ValueType::I32 | ValueType::U32))
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

// <vehicle> 用の上書きルール
#[derive(Default, Debug)]
struct VehicleRule;

impl CodeRule for VehicleRule {
    const TARGET_LABEL: &str = "vehicle files";
}
