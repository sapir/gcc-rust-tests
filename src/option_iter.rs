pub fn test() -> i32 {
    let mut total = 0;
    for i in Some(5) {
        total += i;
    }
    total
}
