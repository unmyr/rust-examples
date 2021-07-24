struct Point<T> {
    #[allow(dead_code)]
    x: T,
    #[allow(dead_code)]
    y: T,
}

fn main() {
    #[allow(unused_variables)]
    let integer = Point { x: 5, y: 10 };
    #[allow(unused_variables)]
    let float = Point { x: 1.0, y: 4.0 };
}
