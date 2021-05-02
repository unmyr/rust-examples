use std::fs::File;

fn main() {
    let _f = File::open("hello.txt").unwrap();
}
