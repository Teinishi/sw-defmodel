define_tag! {
    #[doc = "Represents an element with integer attributes `x`, `y`, and `z`."]
    struct Vec3i {
        "x": i32,
        "y": i32,
        "z": i32,
    }
}

define_tag! {
    #[doc = "Represents an element with float attributes `x`, `y`, and `z`."]
    struct Vec3f {
        "x": f32,
        "y": f32,
        "z": f32,
    }
}

define_tag! {
    #[doc = "Represents an element with int attributes `r`, `g`, and `b`."]
    struct ColorRGB {
        "r": u32,
        "g": u32,
        "b": u32,
    }
}

define_tag! {
    #[doc = "Represents an element with int attributes `r`, `g`, `b`, and `a`."]
    struct ColorRGBA {
        "r": u32,
        "g": u32,
        "b": u32,
        "a": u32,
    }
}

define_tag! {
    #[doc = "Represents an element with 4x4 transform matrix."]
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
