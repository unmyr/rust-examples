use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let v1 = vec![Rc::new(RefCell::new(String::from("hello")))];
    assert_eq!(Rc::strong_count(&v1[0]), 1);
    let v2 = v1.clone();
    assert_eq!(Rc::strong_count(&v1[0]), 2);
    assert_eq!(Rc::ptr_eq(&v1[0], &v2[0]), true);
    v1[0].borrow_mut().push_str(" world");
    println!("v1={:?}, v2={:?}", v1[0].borrow(), v2[0].borrow());
    assert_eq!(*v1[0].borrow(), String::from("hello world"));
    assert_eq!(*v2[0].borrow(), String::from("hello world"));
}