use std::rc::Rc;

fn main() {
    // let mut v1 = vec![Rc::new(String::from("hello"))];
    let v1 = vec![Rc::new(String::from("hello"))];
    assert_eq!(Rc::strong_count(&v1[0]), 1);
    let v2 = v1.clone();
    assert_eq!(Rc::strong_count(&v1[0]), 2);
    assert_eq!(Rc::ptr_eq(&v1[0], &v2[0]), true);
    // v1[0].push_str(" world");  // cannot borrow as mutable
    println!("v1={:?}, v2={:?}", v1, v2);
    assert_eq!(*v1[0], String::from("hello"));
    assert_eq!(*v2[0], String::from("hello"));
}
