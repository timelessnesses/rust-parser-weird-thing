use lol::{built_for_this, returns_slice, SliceStruct};

mod lol;

fn main() {
    // returns_slice()[1];
    built_for_this().0[1];
    // SliceStruct([()]).0[1];
}
