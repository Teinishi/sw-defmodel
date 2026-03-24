mod code;
mod code_rule;
mod node_info;
mod ordered_map;
mod utils;

use code::write_code;
use code_rule::{ChildClassificcation, CodeRule, NamePath};
use node_info::{ChildInfo, NodeInfo, ValueType, analyze_files, print_node};
use std::{
    collections::BTreeSet,
    fmt,
    fs::{self, File},
    io::{self, BufReader, BufWriter, Write},
    path::Path,
};
use utils::ls_xml;

fn main() -> io::Result<()> {
    // test_data/vanilla_definitions 以下を解析
    analyze_and_write(
        "vanilla_definitions",
        "definition_view",
        DefinitionRule::default(),
    )?;

    // test_data/vehicles 以下を解析
    analyze_and_write("vehicles", "vehicle_view", VehicleRule::default())?;

    Ok(())
}

fn analyze_and_write<R: CodeRule>(
    test_data_subdir: &str,
    module_name: &str,
    mut rule: R,
) -> io::Result<()> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    let output_path = manifest_path.join("tmp").join(module_name);
    let cache_path = output_path.with_extension("ron");

    let node: NodeInfo = if cache_path.is_file() {
        // キャッシュがあれば XML を読まずに続行
        println!("Loading XML structure data from cache file: {cache_path:?}");
        ron::de::from_reader(BufReader::new(File::open(cache_path)?)).expect("Failed to load cache")
    } else {
        // キャッシュがなければ test_data から解析
        let xml_path = manifest_path.join("test_data").join(test_data_subdir);
        println!("Analyzing files in {xml_path:?}");
        let node = analyze_files(&ls_xml(xml_path)?.collect::<Vec<_>>());

        // キャッシュに保存
        let cache_str =
            ron::ser::to_string_pretty(&node, ron::ser::PrettyConfig::new().compact_arrays(true))
                .expect("Failed to serialize");
        fs::create_dir_all(cache_path.parent().unwrap())?;
        File::create(cache_path)?.write_all(cache_str.as_bytes())?;

        node
    };

    // log ファイルに記録
    fs::create_dir_all(output_path.parent().unwrap())?;
    let mut f = BufWriter::new(File::create(output_path.with_extension("log"))?);
    print_node(&mut f, &node, 0)?;

    // Rust コードを出力
    let output_path = output_path.with_extension("rs");
    println!("Writing to {output_path:?}");
    let code = write_code(&node, &mut rule);
    let mut f = BufWriter::new(File::create(output_path)?);
    f.write_all(code.as_bytes())?;

    Ok(())
}

// <definition> 用の上書きルール
#[derive(Default, Debug)]
struct DefinitionRule {
    generic_tags: BTreeSet<GenericTag>,
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

        if let Some(g) = is_generic(&info.inner().attributes) {
            self.generic_tags.insert(g);
            Some(ChildClassificcation::unique_inline(g.as_str()))
        } else {
            None
        }
    }

    fn finalize<W: fmt::Write>(&mut self, f1: &mut W, _f2: &mut W) -> fmt::Result {
        write_import(f1, &self.generic_tags)
    }
}

// <vehicle> 用の上書きルール
#[derive(Default, Debug)]
struct VehicleRule {
    generic_tags: BTreeSet<GenericTag>,
}

impl CodeRule for VehicleRule {
    const TARGET_LABEL: &str = "vehicle files";

    fn override_child(
        &mut self,
        path: &NamePath,
        info: &ChildInfo,
    ) -> Option<ChildClassificcation> {
        match path.join_str("/").as_str() {
            "vehicle/bodies/body/components/c/o/microprocessor_definition/group/components/c/object/out1" =>
            {
                return Some(ChildClassificcation::unique());
            }
            "vehicle/bodies/body/components/c/o/display_1/col_extra/c"
            | "vehicle/bodies/body/components/c/o/display_2/col_extra/c"
            | "vehicle/bodies/body/components/c/o/display_3/col_extra/c"
            | "vehicle/bodies/body/components/c/o/display_4/col_extra/c" => {
                return Some(ChildClassificcation::list());
            }
            _ => {}
        }

        if let Some(g) = is_generic(&info.inner().attributes) {
            self.generic_tags.insert(g);
            Some(ChildClassificcation::unique_inline(g.as_str()))
        } else {
            None
        }
    }

    fn finalize<W: fmt::Write>(&mut self, f1: &mut W, _f2: &mut W) -> fmt::Result {
        write_import(f1, &self.generic_tags)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum GenericTag {
    Vec3i,
    Vec3f,
    ColorRGB,
    ColorRGBA,
}

impl GenericTag {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Vec3i => "Vec3i",
            Self::Vec3f => "Vec3f",
            Self::ColorRGB => "ColorRGB",
            Self::ColorRGBA => "ColorRGBA",
        }
    }
}

fn is_generic(
    attrs: &ordered_map::OrderedMap<String, node_info::AttributeInfo>,
) -> Option<GenericTag> {
    if (1..=3).contains(&attrs.len())
        && attrs
            .iter()
            .all(|(name, _)| matches!(name.as_str(), "x" | "y" | "z"))
    {
        if attrs.iter().all(|(_, a)| matches!(a.ty(), ValueType::F32)) {
            return Some(GenericTag::Vec3f);
        } else if attrs
            .iter()
            .all(|(_, a)| matches!(a.ty(), ValueType::I32 | ValueType::U32))
        {
            return Some(GenericTag::Vec3i);
        }
    } else if (1..=4).contains(&attrs.len())
        && attrs.iter().all(|(name, a)| {
            matches!(name.as_str(), "r" | "g" | "b" | "a") && matches!(a.ty(), ValueType::U32)
        })
    {
        if attrs.iter().any(|(name, _)| name == "a") {
            return Some(GenericTag::ColorRGBA);
        } else {
            return Some(GenericTag::ColorRGB);
        }
    }
    None
}

fn write_import<W: fmt::Write>(f: &mut W, generic_tags: &BTreeSet<GenericTag>) -> fmt::Result {
    if !generic_tags.is_empty() {
        write!(f, "pub use super::generic_view::{{")?;
        for (i, g) in generic_tags.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", g.as_str())?;
        }
        writeln!(f, "}};")?;
        writeln!(f)?;
    }

    Ok(())
}
