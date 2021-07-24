struct MyBox<T>(T);

impl<T> MyBox<T> {
    #[allow(dead_code)]
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {}
