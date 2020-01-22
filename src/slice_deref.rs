pub struct Bar<'a> {
    y: &'a [&'a [i32]],
}

pub struct Foo<'a> {
    x: Bar<'a>,
}

fn do_test_slice_deref(x: Foo) -> i32 {
    x.x.y[0][0]
}

pub fn test() -> i32 {
    do_test_slice_deref(Foo {
        x: Bar { y: &[&[1, 2, 3]] },
    })
}
