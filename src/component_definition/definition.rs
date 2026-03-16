use crate::Surface;

define_attributes! {
    "definition" => Definition {
        "name": String,
        "category": u32,
    }
}

impl_child_list!(Definition {
    "surfaces" => surfaces: [Surface],
    "buoyancy_surfaces" => buoyancy_surfaces: [Surface],
});
