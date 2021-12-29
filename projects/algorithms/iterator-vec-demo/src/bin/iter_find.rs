fn main() {
    let strings = vec!["one", "two", "three"];

    let item = strings.iter().find(|&r| *r == "two");
    println!("item={:?}, strings={:?}", item, strings);
    assert_eq!(item, Some(&"two"));

    let item = strings.into_iter().find(|&r| r == "two");
    println!("item={:?}, strings=n/a(moved)", item);
    assert_eq!(item, Some("two"));
}
