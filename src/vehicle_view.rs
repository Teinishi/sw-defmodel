pub use super::generic_view::{Vec3i, Vec3f, ColorRGB, ColorRGBA};

define_root! {
    #[doc = "Represents vehicle files."]
    struct VehicleDocument {
        <vehicle> => Vehicle
    }
}

define_tag! {
    #[doc = "Represents `<vehicle>` tag in vehicle files."]
    struct Vehicle {
        "data_version": u32,
        "is_modded": bool,
        "is_static": bool,
        "bodies_id": u32,
    }
}
define_unique_children!(Vehicle {
    <editor_placement_offset>: Vec3f,
    <authors>: Authors,
});
define_lists!(Vehicle {
    <bodies>: [<body>: Body],
    <logic_node_links>: [<logic_node_link>: LogicNodeLink],
});

define_tag! {
    #[doc = "Represents `<authors>` tag in vehicle files."]
    struct Authors {}
}
define_unique_children!(Authors {
    <author>: Author,
});

define_tag! {
    #[doc = "Represents `<author>` tag in vehicle files."]
    struct Author {
        "steam_id": u64,
        "username": String,
    }
}

define_tag! {
    #[doc = "Represents `<body>` tag in vehicle files."]
    struct Body {
        "unique_id": u32,
    }
}
define_unique_children!(Body {
    <initial_local_transform>: InitialLocalTransform,
    <local_transform>: LocalTransform,
});
define_lists!(Body {
    <components>: [<c>: BodyComponentsC],
});

define_tag! {
    #[doc = "Represents `<initial_local_transform>` tag in vehicle files."]
    struct InitialLocalTransform {
        "00" => attr_00: u32,
        "01" => attr_01: u32,
        "02" => attr_02: u32,
        "03" => attr_03: u32,
        "10" => attr_10: u32,
        "11" => attr_11: u32,
        "12" => attr_12: u32,
        "13" => attr_13: u32,
        "20" => attr_20: u32,
        "21" => attr_21: u32,
        "22" => attr_22: u32,
        "23" => attr_23: u32,
        "30" => attr_30: u32,
        "31" => attr_31: u32,
        "32" => attr_32: u32,
        "33" => attr_33: u32,
    }
}

define_tag! {
    #[doc = "Represents `<local_transform>` tag in vehicle files."]
    struct LocalTransform {
        "00" => attr_00: u32,
        "01" => attr_01: u32,
        "02" => attr_02: u32,
        "03" => attr_03: u32,
        "10" => attr_10: u32,
        "11" => attr_11: u32,
        "12" => attr_12: u32,
        "13" => attr_13: u32,
        "20" => attr_20: u32,
        "21" => attr_21: u32,
        "22" => attr_22: u32,
        "23" => attr_23: u32,
        "30" => attr_30: u32,
        "31" => attr_31: u32,
        "32" => attr_32: u32,
        "33" => attr_33: u32,
    }
}

define_tag! {
    #[doc = "Represents `<c>` tag in vehicle files."]
    struct BodyComponentsC {
        "d": String,
        "t": u32,
    }
}
define_unique_children!(BodyComponentsC {
    <o>: ComponentsCO,
});

define_tag! {
    #[doc = "Represents `<o>` tag in vehicle files."]
    struct ComponentsCO {
        "r": String,
        "bc": String,
        "bc2": String,
        "bc3": String,
        "ac": String,
        "sc": String,
        "scale": u32,
        "spawn_rod": bool,
        "spring_factor": u32,
        "ai_type": u32,
        "blade_count": u32,
        "blade_pitch": u32,
        "blade_length": f32,
        "coal_fill": u32,
        "current_tick": u32,
        "custom_name": String,
        "control_mode_0": u32,
        "decimal_point_pos": u32,
        "audio_data": String,
        "flare_type": u32,
        "flare_color": u32,
        "fluid_type": u32,
        "fuel_factor": f32,
        "gc": String,
        "gca": String,
        "gear_ratio_1": u32,
        "gear_ratio_2": u32,
        "grip_factor": f32,
        "hold_duration": f32,
        "default_state": bool,
        "input_ch_1": u32,
        "input_ch_2": u32,
        "input_ch_3": u32,
        "interactive_default_state": bool,
        "fluid_filter": u32,
        "fluid_fill": f32,
        "m_fov_x": f32,
        "m_fov_y": f32,
        "m_pitch_angle": f32,
        "m_sweep_mode": u32,
        "max_force_scalar": f32,
        "max_force_scale": f32,
        "muzzle_velocity": u32,
        "gear_ratio": u32,
        "input_velocity": f32,
        "ordinance_type": u32,
        "property_ammo_damage": u32,
        "property_ammo_type": u32,
        "rps_limit": u32,
        "sensitivity": f32,
        "spawn_charge": f32,
        "stiffness_factor": f32,
        "damping_factor": f32,
        "throttle_min": f32,
        "throttle_max": f32,
        "timer_scalar_1": f32,
        "timer_scalar_2": f32,
        "tire_type": u32,
        "trigger": u32,
        "trigger_label": String,
        "func_type": u32,
        "hotkey_0": u32,
        "hotkey_0_label": String,
        "hotkey_1": u32,
        "hotkey_1_label": String,
        "hotkey_2": u32,
        "hotkey_2_label": String,
        "hotkey_3": u32,
        "hotkey_3_label": String,
        "hotkey_4": u32,
        "hotkey_4_label": String,
        "hotkey_5": u32,
        "hotkey_5_label": String,
        "control_mode_0_label": String,
        "control_mode_1": u32,
        "control_mode_1_label": String,
        "control_mode_2": u32,
        "control_mode_2_label": String,
        "control_mode_3": u32,
        "control_mode_3_label": String,
        "is_infrared": bool,
        "lss_mode": u32,
        "property_text": String,
        "radar_fov": f32,
        "sensor_radius": f32,
        "sensor_type": u32,
        "sensor_mode": u32,
        "val_1_name": String,
        "val_2_name": String,
        "volume": f32,
        "pitch": f32,
        "wheel_size": f32,
        "double_wheel": bool,
        "tyre_pressure": f32,
    }
}
define_unique_children!(ComponentsCO {
    <microprocessor_definition>: MicroprocessorDefinition,
    <vp>: Vec3i,
    <cc0>: ColorRGBA,
    <cc1>: ColorRGBA,
    <cc2>: ColorRGBA,
    <cc3>: ColorRGBA,
    <cc4>: ColorRGBA,
    <cc5>: ColorRGBA,
    <cc6>: ColorRGBA,
    <cc7>: ColorRGBA,
    <cc8>: ColorRGBA,
    <cc9>: ColorRGBA,
    <cc10>: ColorRGBA,
    <cc11>: ColorRGBA,
    <cc12>: ColorRGBA,
    <cc13>: ColorRGBA,
    <cc14>: ColorRGBA,
    <cc15>: ColorRGBA,
    <cc16>: ColorRGBA,
    <cc17>: ColorRGBA,
    <cc18>: ColorRGBA,
    <cc19>: ColorRGBA,
    <cc20>: ColorRGBA,
    <cc21>: ColorRGBA,
    <cc22>: ColorRGBA,
    <cc23>: ColorRGBA,
    <cc24>: ColorRGBA,
    <cc25>: ColorRGBA,
    <cc26>: ColorRGBA,
    <cc27>: ColorRGBA,
    <cc28>: ColorRGBA,
    <cc29>: ColorRGBA,
    <cc30>: ColorRGBA,
    <cc31>: ColorRGBA,
    <cc32>: ColorRGBA,
    <cc33>: ColorRGBA,
    <cc34>: ColorRGBA,
    <cc35>: ColorRGBA,
    <cc36>: ColorRGBA,
    <cc37>: ColorRGBA,
    <cc38>: ColorRGBA,
    <cc39>: ColorRGBA,
    <cc40>: ColorRGBA,
    <cc41>: ColorRGBA,
    <cc42>: ColorRGBA,
    <cc43>: ColorRGBA,
    <cc44>: ColorRGBA,
    <cc45>: ColorRGBA,
    <cc46>: ColorRGBA,
    <cc47>: ColorRGBA,
    <cc48>: ColorRGBA,
    <cc49>: ColorRGBA,
    <cc50>: ColorRGBA,
    <cc51>: ColorRGBA,
    <cc52>: ColorRGBA,
    <cc53>: ColorRGBA,
    <cc54>: ColorRGBA,
    <cc55>: ColorRGBA,
    <cc56>: ColorRGBA,
    <cc57>: ColorRGBA,
    <cc58>: ColorRGBA,
    <cc59>: ColorRGBA,
    <cc60>: ColorRGBA,
    <cc61>: ColorRGBA,
    <cc62>: ColorRGBA,
    <cc63>: ColorRGBA,
    <cc64>: ColorRGBA,
    <cc65>: ColorRGBA,
    <cc66>: ColorRGBA,
    <cc67>: ColorRGBA,
    <cc68>: ColorRGBA,
    <cc69>: ColorRGBA,
    <cc70>: ColorRGBA,
    <cc71>: ColorRGBA,
    <cc72>: ColorRGBA,
    <cc73>: ColorRGBA,
    <cc74>: ColorRGBA,
    <cc75>: ColorRGBA,
    <cc76>: ColorRGBA,
    <cc77>: ColorRGBA,
    <cc78>: ColorRGBA,
    <cc79>: ColorRGBA,
    <cc80>: ColorRGBA,
    <delta_damping>: Vec3f,
    <display_1>: Display1,
    <display_2>: Display2,
    <display_3>: Display3,
    <display_4>: Display4,
    <impact_sensor_threshold>: ImpactSensorThreshold,
    <input_on_off>: InputOnOff,
    <m_sweep_limit>: MSweepLimit,
    <m_sweep_speed>: MSweepSpeed,
    <min_value>: MinValue,
    <max_value>: MaxValue,
    <property_output_float_val>: PropertyOutputFloatVal,
    <min_threshold>: MinThreshold,
    <max_threshold>: MaxThreshold,
    <pid_controller_ki>: PidControllerKi,
    <pid_controller_kp>: PidControllerKp,
    <pid_controller_kd>: PidControllerKd,
    <pid_controller_max_error>: PidControllerMaxError,
    <exp>: Exp,
    <min_lever_value>: MinLeverValue,
    <max_lever_value>: MaxLeverValue,
    <starting_lever_value>: StartingLeverValue,
    <trim_x_display>: TrimXDisplay,
    <trim_y_display>: TrimYDisplay,
    <trim_z_display>: TrimZDisplay,
    <trim_w_display>: TrimWDisplay,
    <axis_sensitivity>: AxisSensitivity,
});
define_lists!(ComponentsCO {
    <logic_slots>: [<slot>: Slot],
});

define_tag! {
    #[doc = "Represents `<microprocessor_definition>` tag in vehicle files."]
    struct MicroprocessorDefinition {
        "name": String,
        "description": String,
        "width": u32,
        "length": u32,
        "id_counter": u32,
        "id_counter_node": u32,
        "transform_index": u32,
        "sym0": u32,
        "sym1": u32,
        "sym2": u32,
        "sym3": u32,
        "sym4": u32,
        "sym5": u32,
        "sym6": u32,
        "sym7": u32,
        "sym8": u32,
        "sym9": u32,
        "sym10": u32,
        "sym11": u32,
        "sym12": u32,
        "sym13": u32,
        "sym14": u32,
        "sym15": u32,
    }
}
define_unique_children!(MicroprocessorDefinition {
    <group>: Group,
});
define_lists!(MicroprocessorDefinition {
    <nodes>: [<n>: NodesN],
});

define_tag! {
    #[doc = "Represents `<n>` tag in vehicle files."]
    struct NodesN {
        "id": u32,
        "component_id": u32,
    }
}
define_unique_children!(NodesN {
    <node>: Node,
});

define_tag! {
    #[doc = "Represents `<node>` tag in vehicle files."]
    struct Node {
        "label": String,
        "mode": u32,
        "type" => type_attr: u32,
        "description": String,
    }
}
define_unique_children!(Node {
    <position>: Vec3i,
});

define_tag! {
    #[doc = "Represents `<group>` tag in vehicle files."]
    struct Group {}
}
define_unique_children!(Group {
    <data>: Data,
    <groups>: Groups,
});
define_lists!(Group {
    <components>: [<c>: GroupComponentsC],
    <components_bridge>: [<c>: ComponentsBridgeC],
});

define_tag! {
    #[doc = "Represents `<data>` tag in vehicle files."]
    struct Data {
        "type" => type_attr: i32,
    }
}
define_unique_children!(Data {
    <inputs>: Inputs,
    <outputs>: Outputs,
});

define_tag! {
    #[doc = "Represents `<inputs>` tag in vehicle files."]
    #[expect(dead_code)]
    struct Inputs {}
}

define_tag! {
    #[doc = "Represents `<outputs>` tag in vehicle files."]
    #[expect(dead_code)]
    struct Outputs {}
}

define_tag! {
    #[doc = "Represents `<c>` tag in vehicle files."]
    struct GroupComponentsC {
        "type" => type_attr: u32,
    }
}
define_unique_children!(GroupComponentsC {
    <object>: ComponentsCObject,
});

define_tag! {
    #[doc = "Represents `<object>` tag in vehicle files."]
    struct ComponentsCObject {
        "id": u32,
        "count": u32,
        "ct": f32,
        "dt": f32,
        "e" => e_attr: String,
        "l": String,
        "memory": i32,
        "n" => n_attr: String,
        "name": String,
        "offset": i32,
        "on": String,
        "off": String,
        "m": u32,
        "script": String,
        "u": u32,
        "v" => v_attr: String,
        "i" => i_attr: i32,
    }
}
define_unique_children!(ComponentsCObject {
    <pos>: Vec3f,
    <inc>: Inc,
    <in1>: In1,
    <in2>: In2,
    <in3>: In3,
    <in4>: In4,
    <in5>: In5,
    <in6>: In6,
    <in7>: In7,
    <in8>: In8,
    <in9>: In9,
    <in10>: In10,
    <in11>: In11,
    <in12>: In12,
    <in13>: In13,
    <in14>: In14,
    <in15>: In15,
    <in16>: In16,
    <in17>: In17,
    <in18>: In18,
    <in19>: In19,
    <in20>: In20,
    <in21>: In21,
    <in22>: In22,
    <in23>: In23,
    <in24>: In24,
    <in25>: In25,
    <in26>: In26,
    <in27>: In27,
    <in28>: In28,
    <in29>: In29,
    <in30>: In30,
    <in31>: In31,
    <in32>: In32,
    <inoff>: Inoff,
    <min>: Min,
    <max>: Max,
    <int>: Int,
    <out1>: Out1,
    <out2>: Out2,
    <e> => e_el: ObjectE,
    <kp>: Kp,
    <ki>: Ki,
    <kd>: Kd,
    <n> => n_el: ObjectN,
    <r>: ObjectR,
    <i> => i_el: ObjectI,
    <v> => v_el: ItemsIV,
});
define_lists!(ComponentsCObject {
    <items>: [<i>: ItemsI],
});

define_tag! {
    #[doc = "Represents `<inc>` tag in vehicle files."]
    struct Inc {
        "component_id": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in1>` tag in vehicle files."]
    struct In1 {
        "component_id": u32,
        "node_index": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in2>` tag in vehicle files."]
    struct In2 {
        "component_id": u32,
        "disabled": bool,
        "node_index": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in3>` tag in vehicle files."]
    struct In3 {
        "component_id": u32,
        "disabled": bool,
        "node_index": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in4>` tag in vehicle files."]
    struct In4 {
        "component_id": u32,
        "disabled": bool,
        "node_index": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in5>` tag in vehicle files."]
    struct In5 {
        "component_id": u32,
        "disabled": bool,
        "node_index": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in6>` tag in vehicle files."]
    struct In6 {
        "component_id": u32,
        "disabled": bool,
        "node_index": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in7>` tag in vehicle files."]
    struct In7 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in8>` tag in vehicle files."]
    struct In8 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in9>` tag in vehicle files."]
    struct In9 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in10>` tag in vehicle files."]
    struct In10 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in11>` tag in vehicle files."]
    struct In11 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in12>` tag in vehicle files."]
    struct In12 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in13>` tag in vehicle files."]
    struct In13 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in14>` tag in vehicle files."]
    struct In14 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in15>` tag in vehicle files."]
    struct In15 {
        "component_id": u32,
        "disabled": bool,
        "node_index": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in16>` tag in vehicle files."]
    struct In16 {
        "component_id": u32,
        "disabled": bool,
        "node_index": u32,
    }
}

define_tag! {
    #[doc = "Represents `<in17>` tag in vehicle files."]
    struct In17 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in18>` tag in vehicle files."]
    struct In18 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in19>` tag in vehicle files."]
    struct In19 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in20>` tag in vehicle files."]
    struct In20 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in21>` tag in vehicle files."]
    struct In21 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in22>` tag in vehicle files."]
    struct In22 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in23>` tag in vehicle files."]
    struct In23 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in24>` tag in vehicle files."]
    struct In24 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in25>` tag in vehicle files."]
    struct In25 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in26>` tag in vehicle files."]
    struct In26 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in27>` tag in vehicle files."]
    struct In27 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in28>` tag in vehicle files."]
    struct In28 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in29>` tag in vehicle files."]
    struct In29 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in30>` tag in vehicle files."]
    struct In30 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in31>` tag in vehicle files."]
    struct In31 {
        "component_id": u32,
        "disabled": bool,
    }
}

define_tag! {
    #[doc = "Represents `<in32>` tag in vehicle files."]
    struct In32 {
        "component_id": u32,
    }
}

define_tag! {
    #[doc = "Represents `<inoff>` tag in vehicle files."]
    struct Inoff {
        "component_id": u32,
    }
}

define_tag! {
    #[doc = "Represents `<min>` tag in vehicle files."]
    struct Min {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<max>` tag in vehicle files."]
    struct Max {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<int>` tag in vehicle files."]
    struct Int {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<out1>` tag in vehicle files."]
    #[expect(dead_code)]
    struct Out1 {}
}

define_tag! {
    #[doc = "Represents `<out2>` tag in vehicle files."]
    #[expect(dead_code)]
    struct Out2 {}
}

define_tag! {
    #[doc = "Represents `<e>` tag in vehicle files."]
    struct ObjectE {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<i>` tag in vehicle files."]
    struct ItemsI {
        "l": String,
    }
}
define_unique_children!(ItemsI {
    <v>: ItemsIV,
});

define_tag! {
    #[doc = "Represents `<v>` tag in vehicle files."]
    struct ItemsIV {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<kp>` tag in vehicle files."]
    struct Kp {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<ki>` tag in vehicle files."]
    struct Ki {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<kd>` tag in vehicle files."]
    struct Kd {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<n>` tag in vehicle files."]
    struct ObjectN {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<r>` tag in vehicle files."]
    struct ObjectR {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<i>` tag in vehicle files."]
    struct ObjectI {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<c>` tag in vehicle files."]
    struct ComponentsBridgeC {
        "type" => type_attr: u32,
    }
}
define_unique_children!(ComponentsBridgeC {
    <object>: ComponentsBridgeCObject,
});

define_tag! {
    #[doc = "Represents `<object>` tag in vehicle files."]
    struct ComponentsBridgeCObject {
        "id": u32,
    }
}
define_unique_children!(ComponentsBridgeCObject {
    <pos>: Vec3f,
    <in1>: In1,
    <out1>: Out1,
});

define_tag! {
    #[doc = "Represents `<groups>` tag in vehicle files."]
    #[expect(dead_code)]
    struct Groups {}
}

define_tag! {
    #[doc = "Represents `<slot>` tag in vehicle files."]
    struct Slot {
        "editor_connected": u32,
        "modified": bool,
        "value" => value_attr: f32,
    }
}
define_unique_children!(Slot {
    <value> => value_el: SlotValue,
});

define_tag! {
    #[doc = "Represents `<value>` tag in vehicle files."]
    #[expect(dead_code)]
    struct SlotValue {}
}

define_tag! {
    #[doc = "Represents `<display_1>` tag in vehicle files."]
    struct Display1 {
        "type" => type_attr: u32,
        "name": String,
        "channel": u32,
        "mode": u32,
        "mode2": u32,
        "rot": u32,
    }
}
define_unique_children!(Display1 {
    <col>: ColorRGB,
    <min>: Min,
    <max>: Max,
    <col_extra>: ColExtra,
});

define_tag! {
    #[doc = "Represents `<col_extra>` tag in vehicle files."]
    struct ColExtra {
        "size": u32,
    }
}
define_lists!(ColExtra {
    <c>: [<value>: ColExtraCValue],
});

define_tag! {
    #[doc = "Represents `<value>` tag in vehicle files."]
    struct ColExtraCValue {
        "r": u32,
        "g": u32,
        "b": u32,
    }
}

define_tag! {
    #[doc = "Represents `<display_2>` tag in vehicle files."]
    struct Display2 {
        "type" => type_attr: u32,
        "name": String,
        "channel": u32,
        "mode": u32,
        "mode2": u32,
        "rot": u32,
    }
}
define_unique_children!(Display2 {
    <col>: ColorRGB,
    <min>: Min,
    <max>: Max,
    <col_extra>: ColExtra,
});

define_tag! {
    #[doc = "Represents `<display_3>` tag in vehicle files."]
    struct Display3 {
        "type" => type_attr: u32,
        "name": String,
        "channel": u32,
        "mode": u32,
        "mode2": u32,
        "rot": u32,
    }
}
define_unique_children!(Display3 {
    <col>: ColorRGB,
    <min>: Min,
    <max>: Max,
    <col_extra>: ColExtra,
});

define_tag! {
    #[doc = "Represents `<display_4>` tag in vehicle files."]
    struct Display4 {
        "type" => type_attr: u32,
        "name": String,
        "channel": u32,
        "mode": u32,
        "mode2": u32,
        "rot": u32,
    }
}
define_unique_children!(Display4 {
    <col>: ColorRGB,
    <min>: Min,
    <max>: Max,
    <col_extra>: ColExtra,
});

define_tag! {
    #[doc = "Represents `<impact_sensor_threshold>` tag in vehicle files."]
    struct ImpactSensorThreshold {
        "text": u32,
    }
}

define_tag! {
    #[doc = "Represents `<input_on_off>` tag in vehicle files."]
    struct InputOnOff {
        "modified": bool,
        "value": bool,
    }
}

define_tag! {
    #[doc = "Represents `<m_sweep_limit>` tag in vehicle files."]
    struct MSweepLimit {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<m_sweep_speed>` tag in vehicle files."]
    struct MSweepSpeed {
        "text": u32,
        "value": u32,
    }
}

define_tag! {
    #[doc = "Represents `<min_value>` tag in vehicle files."]
    struct MinValue {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<max_value>` tag in vehicle files."]
    struct MaxValue {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<property_output_float_val>` tag in vehicle files."]
    struct PropertyOutputFloatVal {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<min_threshold>` tag in vehicle files."]
    struct MinThreshold {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<max_threshold>` tag in vehicle files."]
    struct MaxThreshold {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<pid_controller_ki>` tag in vehicle files."]
    struct PidControllerKi {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<pid_controller_kp>` tag in vehicle files."]
    struct PidControllerKp {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<pid_controller_kd>` tag in vehicle files."]
    struct PidControllerKd {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<pid_controller_max_error>` tag in vehicle files."]
    struct PidControllerMaxError {
        "text": u32,
    }
}

define_tag! {
    #[doc = "Represents `<exp>` tag in vehicle files."]
    struct Exp {
        "text": u32,
        "value": u32,
    }
}

define_tag! {
    #[doc = "Represents `<min_lever_value>` tag in vehicle files."]
    struct MinLeverValue {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<max_lever_value>` tag in vehicle files."]
    struct MaxLeverValue {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<starting_lever_value>` tag in vehicle files."]
    struct StartingLeverValue {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<trim_x_display>` tag in vehicle files."]
    struct TrimXDisplay {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<trim_y_display>` tag in vehicle files."]
    struct TrimYDisplay {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<trim_z_display>` tag in vehicle files."]
    struct TrimZDisplay {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<trim_w_display>` tag in vehicle files."]
    struct TrimWDisplay {
        "text": f32,
        "value": f32,
    }
}

define_tag! {
    #[doc = "Represents `<axis_sensitivity>` tag in vehicle files."]
    struct AxisSensitivity {
        "x": f32,
        "y": f32,
        "z": f32,
        "w": f32,
    }
}

define_tag! {
    #[doc = "Represents `<logic_node_link>` tag in vehicle files."]
    struct LogicNodeLink {
        "type" => type_attr: u32,
    }
}
define_unique_children!(LogicNodeLink {
    <voxel_pos_0>: Vec3i,
    <voxel_pos_1>: Vec3i,
});

