use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let a_rc: Rc<RefCell<Option<String>>>;
    let b_rc: Rc<RefCell<Option<String>>>;
    let mut a1_weak: Weak<RefCell<Option<String>>>;
    let mut a2_weak: Weak<RefCell<Option<String>>>;
    let mut b_weak: Weak<RefCell<Option<String>>>;
    a_rc = Rc::new(RefCell::new(Some(String::from("a"))));
    b_rc = Rc::new(RefCell::new(Some(String::from("b"))));
    a1_weak = Rc::downgrade(&a_rc);
    a2_weak = Rc::downgrade(&a_rc);
    b_weak = Rc::downgrade(&b_rc);
    std::mem::swap(&mut a1_weak, &mut b_weak);
    std::mem::swap(&mut a2_weak, &mut Rc::downgrade(&b_rc));

    assert_eq!(
        &String::from("b"),
        a1_weak.upgrade().unwrap().borrow().as_ref().unwrap(),
    );
    dbg!(&a1_weak.upgrade().unwrap().borrow().as_ref().unwrap());

    assert_eq!(
        &String::from("b"),
        a2_weak.upgrade().unwrap().borrow().as_ref().unwrap(),
    );
    dbg!(&a2_weak.upgrade().unwrap().borrow().as_ref().unwrap());

    assert_eq!(
        &String::from("a"),
        a_rc.borrow().as_ref().unwrap(),
    );
    dbg!(&a_rc.borrow().as_ref().unwrap());
}
