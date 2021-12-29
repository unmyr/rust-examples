use std::rc::Rc;

fn main() {
    let v1 = Rc::new(vec![String::from("hello")]);
    assert_eq!(Rc::strong_count(&v1), 1);
    let v2 = v1.clone();
    assert_eq!(Rc::strong_count(&v1), 2);
    assert_eq!(Rc::ptr_eq(&v1, &v2), true);
    println!("v1={:?}, v2={:?}", v1, v2);
    assert_eq!(*v1, vec![String::from("hello")]);
    assert_eq!(*v2, vec![String::from("hello")]);
}
