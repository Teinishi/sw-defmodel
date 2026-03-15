xml_enum! {
    enum SurfaceOrientation: u32 {
        XPos = 0,
        XNeg = 1,
        YPos = 2,
        YNeg = 3,
        ZPos = 4,
        ZNeg = 5,
    }
}

element_wrapper! {
    "surface" => Surface {
        "orientation": SurfaceOrientation,
        "rotation": i32,
        "shape": i32,
        "trans_type": i32,
    }
}
