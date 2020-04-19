pub fn test() -> i32 {
    // not using 0 for unwrap value because it might be returned accidentally with bad
    // code generation
    (10..3).next().unwrap_or(1)
}
