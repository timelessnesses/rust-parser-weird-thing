# Rust's compiler bug thing?

Apparently the boundary checks for slice size is not checked under a circumstances for some reasons. If you try to run the code without removing commented out codes you will get error of `out of bounds` error, Rust does support compile time checks for any panic out of bound when accessing slices. But if your slice is in under a Tuple Struct which is returned by a function, the compiler won't check it for you. You can try remove comments and see that the `built_for_this` function doesn't get printed as error when try to compile the code.
