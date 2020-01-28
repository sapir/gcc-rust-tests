pub fn test_just_an_int() -> i32 {
    let mut total = 0;
    for (_, i) in Some((8, &5)) {
        total += *i;
    }
    total
}

// None should occupy a niche
pub fn test_tuple_with_ref() -> i32 {
    let mut total = 0;
    for (_, i) in Some((8, &5)) {
        total += *i;
    }
    total
}
