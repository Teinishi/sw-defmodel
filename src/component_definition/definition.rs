use crate::Surface;

define_attributes! {
    "definition" => Definition {
        "name": String,
        "category": u32,
        "type" => type_attr: u32,
        "mass": f32,
        "value": u32,
        "flags": u64,
        "tags": String,
    }
}

impl_child_list!(Definition {
    "surfaces" => surfaces: [Surface],
    "buoyancy_surfaces" => buoyancy_surfaces: [Surface],
});
