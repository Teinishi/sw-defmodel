mod schema_analyzer;
mod write_macros;
mod write_rule;

use schema_analyzer::{SchemaChild, analyze_schema};
use std::{
    io::{self, Write},
    path::Path,
};
use write_rule::{ChildElementType, SchemaWriteRule};

const MAX_ENUM: usize = 10;

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
