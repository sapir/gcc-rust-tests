pub struct MyStruct {
    a: i32,
    b: i32,
}

pub enum MyEnum {
    Struct { my_struct: MyStruct, x: i32 },
    OnlyInt { y: i32 },
    Array([i32; 20]),
}

fn make_enum1() -> MyEnum {
    MyEnum::Struct {
        my_struct: MyStruct { a: 2, b: 7 },
        x: 13,
    }
}

fn make_enum2() -> MyEnum {
    MyEnum::OnlyInt { y: 5 }
}

fn make_enum3() -> MyEnum {
    MyEnum::Array([9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0])
}

pub fn test1_my_struct_a() -> i32 {
    if let MyEnum::Struct { my_struct, .. } = make_enum1() {
        my_struct.a
    } else {
        -1
    }
}

pub fn test1_my_struct_b() -> i32 {
    if let MyEnum::Struct { my_struct, .. } = make_enum1() {
        my_struct.b
    } else {
        -1
    }
}

pub fn test1_x() -> i32 {
    if let MyEnum::Struct { x, .. } = make_enum1() {
        x
    } else {
        -1
    }
}

pub fn test2() -> i32 {
    if let MyEnum::OnlyInt { y } = make_enum1() {
        y
    } else {
        -1
    }
}

pub fn test3_0() -> i32 {
    if let MyEnum::Array(arr) = make_enum1() {
        arr[0]
    } else {
        -1
    }
}

pub fn test3_1() -> i32 {
    if let MyEnum::Array(arr) = make_enum1() {
        arr[1]
    } else {
        -1
    }
}

pub fn test3_10() -> i32 {
    if let MyEnum::Array(arr) = make_enum1() {
        arr[10]
    } else {
        -1
    }
}
