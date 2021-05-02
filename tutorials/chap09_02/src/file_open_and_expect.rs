use std::fs::File;

fn main() {
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}
