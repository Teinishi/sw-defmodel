use crate::Surface;

define_attributes!(Definition {
    "name": String,
    "category": u32,
    "type" => type_attr: u32,
    "mass": f32,
    "value": u32,
    "flags": u64,
    "tags": String,
});

define_unique_children!(Definition {
    <voxel_min>: VoxelMin,
});

define_lists!(Definition {
    <surfaces>: [<surface>: Surface],
    <buoyancy_surfaces>: [<surface>: Surface],
});

define_attributes!(VoxelMin {
    "x": i32,
    "y": i32,
    "z": i32,
});
