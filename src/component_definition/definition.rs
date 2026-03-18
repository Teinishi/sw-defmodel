use crate::Surface;

define_tag!(Definition {
    "name": String,
    "category": u32,
    "type" => type_attr: u32,
    "mass": f32,
    "value": u32,
    "flags": u64,
    "tags": String,
    "extender_name": enum ExtenderName &str {
        LinearModule = "linear_module",
        LinearCompactModule = "linear_compact_module",
        None = "",
    } at "sw_defmodel::component_definition",
    "button_type": #[doc = "Represents the [button_type][Definition::button_type] XML attribute in [definition][Definition].\n\nA subtype for buttons where [type][Definition::type_attr()] is 8. The value is stored as an integer in XML."] enum ButtonType u32 {
        Push = 0,
        Toggle = 1,
        Key = 2,
        Lockable = 3,
        ThrottleLever = 4,
        SmallKeypad = 5,
        LargeKeypad = 6,
    } at "sw_defmodel::component_definition",
});

define_unique_children!(Definition {
    <voxel_min>: VoxelMin,
});

define_lists!(Definition {
    <surfaces>: [<surface>: Surface],
    <buoyancy_surfaces>: [<surface>: Surface],
});

define_tag!(VoxelMin {
    "x": i32,
    "y": i32,
    "z": i32,
});
