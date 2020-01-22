pub enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T>
where
    T: Copy,
{
    #[inline]
    pub fn make_an_option(x: T) -> Option<T> {
        Option::Some(x)
    }
}

impl Option<i32> {
    pub fn unwrap(self) -> i32 {
        match self {
            Option::None => 0,
            Option::Some(x) => x,
        }
    }
}

pub fn test() -> i32 {
    Option::make_an_option(5).unwrap()
}
