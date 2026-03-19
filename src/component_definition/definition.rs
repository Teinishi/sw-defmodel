use crate::Surface;

define_tag!(
    #[doc = "Represents `<definition>` tag in component definition files.\n\n"]
    struct Definition {
        "name": String,
        "category": u32,
        "type" => type_attr: u32,
        "mass": f32,
        "value": u32,
        "flags": u64,
        "tags": String,
        "extender_name":
            #[doc = ""]
            enum ExtenderName &str {
                LinearModule = "linear_module",
                LinearCompactModule = "linear_compact_module",
                None = "",
            },
        #[doc = "\n\nA subtype for buttons where the [type attribute][Definition::type_attr()] has a value of 8. See [`ButtonType`] for possible values."]
        "button_type":
            #[doc = "A subtype for buttons where the [type attribute][Definition::type_attr()] has a value of 8.\n\n"]
            enum ButtonType u32 {
                Push = 0,
                Toggle = 1,
                Key = 2,
                Lockable = 3,
                ThrottleLever = 4,
                SmallKeypad = 5,
                LargeKeypad = 6,
            },
    }
);

define_unique_children!(Definition {
    <voxel_min>: Vec3i,
    <voxel_max>: Vec3i,
});

define_lists!(Definition {
    <surfaces>: [<surface>: Surface],
    <buoyancy_surfaces>: [<surface>: Surface],
});

define_tag! {
    #[doc = ""]
    struct Vec3i {
        "x": i32,
        "y": i32,
        "z": i32,
    }
}
