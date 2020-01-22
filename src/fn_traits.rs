#![feature(fn_traits, unboxed_closures)]

struct Foo;

impl FnOnce<(i32, i32)> for Foo {
    type Output = i32;

    extern "rust-call" fn call_once(self, args: (i32, i32)) -> Self::Output {
        args.0 + args.1
    }
}

impl FnMut<(i32, i32)> for Foo {
    extern "rust-call" fn call_mut(&mut self, args: (i32, i32)) -> Self::Output {
        args.0 + args.1
    }
}

impl Fn<(i32, i32)> for Foo {
    extern "rust-call" fn call(&self, args: (i32, i32)) -> Self::Output {
        args.0 + args.1
    }
}

pub fn test() -> i32 {
    Foo(2, 3)
}
