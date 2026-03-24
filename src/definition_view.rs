pub use super::generic_view::{Vec3i, Vec3f};

define_root! {
    #[doc = "Represents component definition files."]
    struct DefinitionDocument {
        <definition> => Definition
    }
}

define_tag! {
    #[doc = "Represents `<definition>` tag in component definition files."]
    struct Definition {
        "name": String,
        "category": u32,
        "type" => type_attr: u32,
        "mass": f32,
        "value": u32,
        "flags": u64,
        "tags": String,
        "phys_collision_dampen": u32,
        "audio_filename_start": String,
        "audio_filename_loop": String,
        "audio_filename_end": String,
        "audio_filename_start_b": String,
        "audio_filename_loop_b": String,
        "audio_filename_end_b": String,
        "audio_gain": f32,
        "mesh_data_name": String,
        "mesh_0_name": String,
        "mesh_1_name": String,
        "mesh_2_name": String,
        "mesh_editor_only_name": String,
        "metadata_component_type": u32,
        "block_type": u32,
        "child_name": String,
        "extender_name": String,
        "constraint_type": u32,
        "constraint_axis": u32,
        "constraint_range_of_motion": f32,
        "max_motor_force": f32,
        "max_motor_speed": f32,
        "cable_radius": f32,
        "cable_length": i32,
        "oil_component_type": u32,
        "seat_pose": u32,
        "seat_health_per_sec": u32,
        "seat_type": u32,
        "tool_type": u32,
        "buoy_radius": f32,
        "buoy_factor": f32,
        "buoy_force": f32,
        "force_emitter_max_force": f32,
        "force_emitter_max_vector": f32,
        "force_emitter_default_pitch": u32,
        "force_emitter_blade_height": f32,
        "force_emitter_rotation_speed": f32,
        "force_emitter_blade_physics_length": f32,
        "force_emitter_blade_efficiency": f32,
        "force_emitter_efficiency": f32,
        "engine_max_force": f32,
        "engine_frictionless_force": u32,
        "trans_conn_type": u32,
        "trans_type": u32,
        "wheel_radius": f32,
        "wheel_wishbone_length": f32,
        "wheel_suspension_height": f32,
        "wheel_wishbone_margin": f32,
        "wheel_suspension_offset": f32,
        "wheel_wishbone_offset": f32,
        "wheel_type": u32,
        "button_type": u32,
        "light_intensity": f32,
        "light_range": f32,
        "light_ies_map": String,
        "light_fov": f32,
        "light_type": u32,
        "door_lower_limit": f32,
        "door_upper_limit": f32,
        "door_flipped": bool,
        "custom_door_type": u32,
        "door_side_dist": u32,
        "door_up_dist": u32,
        "dynamic_min_rotation": f32,
        "dynamic_max_rotation": f32,
        "data_logger_component_type": u32,
        "logic_gate_type": u32,
        "logic_gate_subtype": u32,
        "indicator_type": u32,
        "connector_type": u32,
        "magnet_force": f32,
        "gyro_type": u32,
        "reward_tier": u32,
        "revision": u32,
        "rudder_surface_area": f32,
        "m_pump_pressure": f32,
        "pump_pressure": f32,
        "water_component_type": u32,
        "wheel_width": f32,
        "torque_component_type": u32,
        "jet_engine_component_type": u32,
        "particle_speed": f32,
        "inventory_class": u32,
        "inventory_default_item": u32,
        "inventory_type": u32,
        "inventory_default_outfit": u32,
        "electric_type": u32,
        "electric_charge_capacity": u32,
        "electric_magnitude": f32,
        "composite_type": u32,
        "camera_fov_min": f32,
        "camera_fov_max": f32,
        "monitor_border": f32,
        "monitor_inset": f32,
        "weapon_type": u32,
        "weapon_class": u32,
        "weapon_belt_type": u32,
        "weapon_ammo_capacity": u32,
        "weapon_ammo_feed": bool,
        "weapon_barrel_length_voxels": u32,
        "rx_range": u32,
        "rx_length": f32,
        "rocket_type": u32,
        "radar_range": u32,
        "radar_speed": f32,
        "rudder_type": u32,
        "engine_module_type": u32,
        "steam_component_type": u32,
        "steam_component_capacity": f32,
        "nuclear_component_type": u32,
        "radar_type": u32,
        "piston_len": f32,
        "piston_cam": f32,
    }
}
define_unique_children!(Definition {
    <voxel_min>: Vec3i,
    <voxel_max>: Vec3i,
    <voxel_physics_min>: Vec3i,
    <voxel_physics_max>: Vec3i,
    <bb_physics_min>: Vec3f,
    <bb_physics_max>: Vec3f,
    <compartment_sample_pos>: Vec3i,
    <constraint_pos_parent>: Vec3f,
    <constraint_pos_child>: Vec3f,
    <voxel_location_child>: Vec3i,
    <seat_offset>: Vec3f,
    <seat_front>: Vec3i,
    <seat_up>: Vec3i,
    <seat_camera>: SeatCamera,
    <seat_render>: SeatRender,
    <force_dir>: Vec3f,
    <light_position>: Vec3i,
    <light_color>: Vec3f,
    <light_forward>: Vec3i,
    <door_size>: Vec3f,
    <door_normal>: Vec3i,
    <door_side>: Vec3i,
    <door_up>: Vec3i,
    <door_base_pos>: Vec3i,
    <dynamic_body_position>: Vec3i,
    <dynamic_rotation_axes>: Vec3f,
    <dynamic_side_axis>: Vec3f,
    <magnet_offset>: Vec3f,
    <connector_axis>: Vec3i,
    <connector_up>: Vec3i,
    <tooltip_properties>: TooltipProperties,
    <reward_properties>: RewardProperties,
    <seat_exit_position>: Vec3i,
    <particle_direction>: Vec3i,
    <particle_offset>: ParticleOffset,
    <particle_bounds>: ParticleBounds,
    <weapon_breech_position>: Vec3i,
    <weapon_breech_normal>: Vec3i,
    <weapon_cart_position>: Vec3f,
    <weapon_cart_velocity>: WeaponCartVelocity,
    <rope_hook_offset>: RopeHookOffset,
});
define_lists!(Definition {
    <sfx_datas>: [<sfx_data>: SfxData],
    <surfaces>: [<surface>: Surface],
    <buoyancy_surfaces>: [<surface>: Surface],
    <logic_nodes>: [<logic_node>: LogicNode],
    <couplings>: [<coupling>: Coupling],
    <voxels>: [<voxel>: Voxel],
    <jet_engine_connections_prev>: [<j>: JetEngineConnectionsPrevJ],
    <jet_engine_connections_next>: [<j>: JetEngineConnectionsPrevJ],
});

define_tag! {
    #[doc = "Represents `<sfx_data>` tag in component definition files."]
    struct SfxData {
        "sfx_name": String,
        "sfx_range_inner": f32,
        "sfx_range_outer": f32,
        "sfx_priority": f32,
        "sfx_is_underwater_affected": bool,
    }
}
define_lists!(SfxData {
    <sfx_layers>: [<sfx_layer>: SfxLayer],
});

define_tag! {
    #[doc = "Represents `<sfx_layer>` tag in component definition files."]
    struct SfxLayer {
        "sfx_filename_start": String,
        "sfx_filename_loop": String,
        "sfx_filename_end": String,
        "sfx_gain": f32,
        "sfx_loop_start_time": f32,
        "sfx_loop_blend_duration": f32,
        "sfx_volume_fade_speed": f32,
        "sfx_pitch_fade_speed": f32,
    }
}

define_tag! {
    #[doc = "Represents `<surface>` tag in component definition files."]
    struct Surface {
        "orientation": u32,
        "rotation": u32,
        "shape": u32,
        "trans_type": u32,
        "flags": u32,
        "is_reverse_normals": bool,
        "is_two_sided": bool,
    }
}
define_unique_children!(Surface {
    <position>: Vec3i,
});

define_tag! {
    #[doc = "Represents `<logic_node>` tag in component definition files."]
    struct LogicNode {
        "orientation": u32,
        "label": String,
        "mode": u32,
        "type" => type_attr: u32,
        "description": String,
        "flags": u32,
    }
}
define_unique_children!(LogicNode {
    <position>: Vec3i,
});

define_tag! {
    #[doc = "Represents `<coupling>` tag in component definition files."]
    struct Coupling {
        "orientation": u32,
        "alignment": u32,
        "coupling_type": String,
        "coupling_name": String,
        "coupling_gender": u32,
        "alignment_required": bool,
        "allow_bipolar_alignment": bool,
    }
}
define_unique_children!(Coupling {
    <position>: Vec3i,
});

define_tag! {
    #[doc = "Represents `<voxel>` tag in component definition files."]
    struct Voxel {
        "flags": u32,
        "physics_shape": u32,
        "buoy_pipes": u32,
    }
}
define_unique_children!(Voxel {
    <position>: Vec3i,
    <physics_shape_rotation>: PhysicsShapeRotation,
});

define_tag! {
    #[doc = "Represents `<physics_shape_rotation>` tag in component definition files."]
    struct PhysicsShapeRotation {
        "00" => attr_00: i32,
        "01" => attr_01: i32,
        "02" => attr_02: i32,
        "10" => attr_10: i32,
        "11" => attr_11: i32,
        "12" => attr_12: i32,
        "20" => attr_20: i32,
        "21" => attr_21: i32,
        "22" => attr_22: i32,
    }
}

define_tag! {
    #[doc = "Represents `<seat_camera>` tag in component definition files."]
    struct SeatCamera {
        "x": u32,
        "y": f32,
        "z": f32,
    }
}

define_tag! {
    #[doc = "Represents `<seat_render>` tag in component definition files."]
    struct SeatRender {
        "x": u32,
        "y": f32,
        "z": f32,
    }
}

define_tag! {
    #[doc = "Represents `<tooltip_properties>` tag in component definition files."]
    struct TooltipProperties {
        "description": String,
        "short_description": String,
    }
}

define_tag! {
    #[doc = "Represents `<reward_properties>` tag in component definition files."]
    struct RewardProperties {
        "tier": u32,
        "number_rewarded": u32,
    }
}

define_tag! {
    #[doc = "Represents `<j>` tag in component definition files."]
    struct JetEngineConnectionsPrevJ {}
}
define_unique_children!(JetEngineConnectionsPrevJ {
    <pos>: Vec3i,
    <normal>: Vec3i,
});

define_tag! {
    #[doc = "Represents `<particle_offset>` tag in component definition files."]
    struct ParticleOffset {
        "x": u32,
        "y": f32,
        "z": f32,
    }
}

define_tag! {
    #[doc = "Represents `<particle_bounds>` tag in component definition files."]
    struct ParticleBounds {
        "x": f32,
        "y": f32,
        "z": f32,
    }
}

define_tag! {
    #[doc = "Represents `<weapon_cart_velocity>` tag in component definition files."]
    struct WeaponCartVelocity {
        "x": f32,
        "y": f32,
        "z": u32,
    }
}

define_tag! {
    #[doc = "Represents `<rope_hook_offset>` tag in component definition files."]
    struct RopeHookOffset {
        "x": u32,
        "y": f32,
        "z": f32,
    }
}

