pub fn test() -> i32 {
    let mut total = 0;
    for i in &[1, 2, 3, 4, 5] {
        total += *i;
    }
    total
}
