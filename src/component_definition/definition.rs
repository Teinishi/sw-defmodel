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

impl_unique_child!(Definition {
    "voxel_min" => voxel_min: VoxelMin,
});

impl_child_list!(Definition {
    "surfaces" => surfaces: [Surface],
    "buoyancy_surfaces" => buoyancy_surfaces: [Surface],
});

define_attributes! {
    "voxel_min" => VoxelMin {
        "x": i32,
        "y": i32,
        "z": i32,
    }
}
