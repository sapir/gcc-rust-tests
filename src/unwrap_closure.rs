fn do_test(x: Option<i32>, y: i32) -> i32 {
    x.unwrap_or_else(|| y + 1)
}

pub fn test() -> i32 {
    do_test(Some(8), 0)
}
