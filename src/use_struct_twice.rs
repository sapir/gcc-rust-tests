pub struct X(i32);

pub fn test_use_struct_twice() -> i32 {
    let mut x = X(0);
    x.0 = 5;
    x.0
}
