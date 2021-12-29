fn main() {
    let strings = vec!["one", "two", "three"];

    let some_index = strings.iter().position(|&r| r == "two");
    println!("index={:?}, strings={:?}", some_index, strings);
    assert_eq!(some_index, Some(1));

    let some_index = strings.into_iter().position(|r| r == "two");
    println!("index={:?}, strings=n/a(moved)", some_index);
    assert_eq!(some_index, Some(1));
}
