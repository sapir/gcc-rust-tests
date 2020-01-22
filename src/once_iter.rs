pub fn test() -> i32 {
    let mut total = 0;
    for x in std::iter::once(3)
        .chain(std::iter::once(1))
        .chain(std::iter::once(4))
    {
        total += x;
    }
    for i in 0..10 {
        total += i;
    }
    total - 53
}
