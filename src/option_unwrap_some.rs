pub fn test() -> i32 {
    // not using 0 for unwrap value because it might be returned accidentally with bad
    // code generation
    Some(5).unwrap_or(2)
}
