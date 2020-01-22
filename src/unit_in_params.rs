fn bar(x: ()) -> i32 {
    5
}
fn baz(x: i32) {}
pub fn test() -> i32 {
    bar(baz(1)) - 5
}
