pub fn returns_slice() -> [(); 1] {
    [()]
}

pub struct SliceStruct(pub [(); 1]);

pub fn built_for_this() -> SliceStruct {
    SliceStruct([()])
}

fn main() {
    // compiler error
    returns_slice()[1];
    // panics
    built_for_this().0[1];
    // compiler error
    SliceStruct([()]).0[1];
}
