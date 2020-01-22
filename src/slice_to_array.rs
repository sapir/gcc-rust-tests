fn foo(x: &[u8]) -> [u8; 4] {
    [x[3], x[2], x[2], x[3]]
}

pub fn test() -> i32 {
    let arr = &[1, 2, 3, 4];
    i32::from_le_bytes(foo(&arr[..]))
}
