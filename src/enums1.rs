pub enum A {
    I16(i16),
    I32(i32),
}

pub enum B1<T> {
    None,
    Some(T),
}

pub enum B2<T> {
    None,
    Some(T),
}

pub enum C<T> {
    B1(B1<T>),
    B2(B2<T>),
}

impl C<A> {
    fn get_it(self) -> i32 {
        match self {
            C::B1(B1::Some(A::I32(x))) => x,
            C::B2(B2::Some(A::I32(x))) => x,
            _ => 0,
        }
    }
}

fn do_test(c1: C<A>, c2: C<A>) -> i32 {
    c1.get_it() + c2.get_it()
}

pub fn test() -> i32 {
    do_test(C::B1(B1::Some(A::I32(8))), C::B2(B2::Some(A::I16(5))))
}
