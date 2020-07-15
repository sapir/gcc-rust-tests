fn fold<F, Accum, Value, I>(initial: Accum, mut f: F, mut iterable: I) -> Accum
where
    F: FnMut(Accum, Value) -> Accum,
    I: Iterator<Item = Value>,
{
    let mut a = initial;
    while let Some(v) = iterable.next() {
        a = f(a, v);
    }
    a
}

pub fn test() -> i32 {
    fold(0, |a, b| a + b, (0..9).into_iter())
}
