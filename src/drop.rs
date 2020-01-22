struct MyDroppable<'a>(&'a mut i32);

impl<'a> Drop for MyDroppable<'a> {
    fn drop(&mut self) {
        *self.0 += 1;
    }
}

pub fn test() -> i32 {
    let mut x = 5;
    {
        MyDroppable(&mut x);
    }
    x - 6
}
