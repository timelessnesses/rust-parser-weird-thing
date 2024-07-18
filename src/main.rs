pub fn returns_slice() -> [(); 1] {
    [()]
}

pub struct SliceStruct(pub [(); 1]);

pub fn built_for_this() -> SliceStruct {
    SliceStruct([()])
}

fn main() {
    // returns_slice()[1];
    built_for_this().0[1];
    // SliceStruct([()]).0[1];
}
