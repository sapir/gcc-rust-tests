fn do_test(a: i32, b: i32) -> i32 {
    a + b
}

pub fn test() -> i32 {
    // a separate function so that it won't get optimized out
    do_test(1, 2)
}
