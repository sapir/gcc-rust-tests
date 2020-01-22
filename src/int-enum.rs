#[repr(i32)]
enum MyEnum {
    A = 0,
    B,
    C,
}

fn foo(x: MyEnum) -> i32 {
    // Do both match and `as`
    match x {
        MyEnum::A => MyEnum::A as i32,
        MyEnum::B => MyEnum::B as i32,
        MyEnum::C => MyEnum::C as i32,
    }
}

pub fn test() -> i32 {
    foo(MyEnum::B) - 1
}
