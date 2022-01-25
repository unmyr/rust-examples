use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let m: Rc<Option<RefCell<String>>>;

    let s1 = String::from("hello");
    m = Rc::new(Some(RefCell::new(s1.clone())));

    assert_eq!(
        &Rc::new(Some(RefCell::new(s1.clone()))),
        &m
    );
    assert_eq!(
        &Some(RefCell::new(s1.clone())),
        m.as_ref()
    );
    assert_eq!(
        Some(&RefCell::new(s1.clone())),
        m.as_ref().as_ref()
    );
    assert_eq!(
        &RefCell::new(s1.clone()),
        m.as_ref().as_ref().unwrap()
    );
    assert_eq!(
        s1,
        m.as_ref().as_ref().unwrap().borrow().clone()
    );
    m.as_ref().as_ref().unwrap().borrow_mut().push_str(" world!");
    assert_eq!(
        String::from("hello world!"),
        m.as_ref().as_ref().unwrap().borrow().clone()
    );
}