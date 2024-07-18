pub fn returns_slice() -> [(); 1] {
    [()]
}

pub struct SliceStruct(pub [(); 1]);

pub fn built_for_this() -> SliceStruct {
    SliceStruct([()])
}
