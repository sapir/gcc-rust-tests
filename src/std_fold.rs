pub fn test() -> i32 {
    (0..9).into_iter().fold(0, |a, b| a + b)
}
