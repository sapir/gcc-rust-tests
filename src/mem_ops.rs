pub fn test_replace_return_old() -> i32 {
    let mut x = 1;
    std::mem::replace(&mut x, 8)
}

pub fn test_replace_return_new() -> i32 {
    let mut x = 1;
    let _ = std::mem::replace(&mut x, 8);
    x
}

pub fn test_swap_return_first() -> i32 {
    let mut x = 1;
    let mut y = 2;
    std::mem::swap(&mut x, &mut y);
    x
}

pub fn test_swap_return_second() -> i32 {
    let mut x = 1;
    let mut y = 2;
    std::mem::swap(&mut x, &mut y);
    y
}
