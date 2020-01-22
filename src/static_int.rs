static MY_STATIC: i32 = 5;

#[inline(never)]
pub fn get_it(x: &i32) -> i32 {
    *x
}

pub fn test() -> i32 {
    // Accessing MY_STATIC is currently optimized out, but passing it to a function currently
    // isn't.
    get_it(&MY_STATIC)
}
