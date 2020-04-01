pub fn test_no_args() -> i32 {
    let my_closure = || 5;
    my_closure()
}

/// Closure upvars struct is expected to have a Scalar ABI layout
pub fn test_1args() -> i32 {
    let my_closure = |a| a;
    my_closure(1)
}

/// Closure upvars struct is expected to have a ScalarPair ABI layout
pub fn test_2args() -> i32 {
    let my_closure = |a, b| a + b;
    my_closure(1, 2)
}

pub fn test_3args() -> i32 {
    let my_closure = |a, b, c| a + b + c;
    my_closure(1, 2, 3)
}
