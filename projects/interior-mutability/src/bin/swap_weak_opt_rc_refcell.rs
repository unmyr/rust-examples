use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let a_rc: Option<Rc<RefCell<String>>>;
    let b_rc: Option<Rc<RefCell<String>>>;
    let mut a1_weak: Option<Weak<RefCell<String>>>;
    let mut a2_weak: Option<Weak<RefCell<String>>>;
    let mut b_weak: Option<Weak<RefCell<String>>>;
    a_rc = Some(Rc::new(RefCell::new(String::from("a"))));
    b_rc = Some(Rc::new(RefCell::new(String::from("b"))));
    a1_weak = Some(Rc::downgrade(a_rc.as_ref().unwrap()));
    a2_weak = Some(Rc::downgrade(a_rc.as_ref().unwrap()));
    b_weak = Some(Rc::downgrade(b_rc.as_ref().unwrap()));

    std::mem::swap(&mut a1_weak, &mut b_weak);
    a2_weak.replace(
        Rc::downgrade(
            &Rc::clone(b_rc.as_ref().unwrap())
        )
    );

    assert_eq!(
        String::from("b"),
        a1_weak.as_ref().unwrap().upgrade().unwrap().borrow().clone(),
    );
    dbg!(&a1_weak.as_ref().unwrap().upgrade().unwrap().borrow());

    assert_eq!(
        String::from("b"),
        a2_weak.as_ref().unwrap().upgrade().unwrap().borrow().clone(),
    );
    dbg!(&a2_weak.as_ref().unwrap().upgrade().unwrap().borrow());

    assert_eq!(
        String::from("a"),
        a_rc.as_ref().unwrap().borrow().clone(),
    );
    dbg!(&a_rc.as_ref().unwrap().borrow().clone());
}
