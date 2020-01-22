pub fn test() -> i32 {
    let x = X;
    let y = &x as &dyn MyTrait;
    call_foo(y)
}

#[inline(never)]
fn call_foo(y: &dyn MyTrait) -> i32 {
    y.foo()
}

pub trait MyTrait {
    fn foo(&self) -> i32;
}

pub struct X;

impl MyTrait for X {
    fn foo(&self) -> i32 {
        5
    }
}
