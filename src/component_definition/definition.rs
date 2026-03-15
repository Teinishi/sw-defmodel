use crate::Surface;

element_wrapper! {
    "definition" => Definition {
        "name" => name: String,
        "category" => category: u32,
    }
}

impl_child_list!(Definition {
    "surfaces" => surfaces: [Surface],
    "buoyancy_surfaces" => buoyancy_surfaces: [Surface],
});
