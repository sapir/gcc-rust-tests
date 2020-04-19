#[repr(i32)]
enum SingleVariantEmptyEnum {
    A = 8,
}

#[repr(i32)]
enum SingleVariantStructEnum {
    A(i32),
}

fn foo(x: SingleVariantEmptyEnum, y: SingleVariantStructEnum) -> i32 {
    let SingleVariantStructEnum::A(y) = y;
    x as i32 + y
}

pub fn test() -> i32 {
    foo(SingleVariantEmptyEnum::A, SingleVariantStructEnum::A(5))
}
