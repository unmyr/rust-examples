use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let v1 = Rc::new(RefCell::new(vec![String::from("hello")]));
    assert_eq!(Rc::strong_count(&v1), 1);
    let v2 = Rc::clone(&v1);
    assert_eq!(Rc::strong_count(&v1), 2);
    assert!(Rc::ptr_eq(&v1, &v2));
    v1.borrow_mut()[0].push_str(" world");
    println!("v1={:?}, v2={:?}", v1.borrow(), v2.borrow());
    assert_eq!(*v1.borrow(), vec![String::from("hello world")]);
    assert_eq!(*v2.borrow(), vec![String::from("hello world")]);
}
