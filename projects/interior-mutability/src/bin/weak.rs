use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let alice: Rc<RefCell<Option<&u8>>>;
    let bob: Weak<RefCell<Option<&u8>>>;
    let a = 1;

    alice = Rc::new(RefCell::new(Some(&a)));
    bob = Rc::downgrade(&alice);
    assert_eq!(alice.borrow().clone(), Some(&1));
    assert_eq!(bob.upgrade().unwrap().borrow().clone(), Some(&1));
    assert_eq!(1, Rc::strong_count(&alice));
    assert_eq!(1, Rc::weak_count(&alice));
    println!("alice={:?}, bob={:?}", alice.borrow(), bob.upgrade());

    bob.upgrade().unwrap().replace(None);
    println!("alice={:?}, bob={:?}", alice.borrow(), bob.upgrade());
    assert_eq!(alice.borrow().clone(), None);
    assert_eq!(bob.upgrade().unwrap().borrow().clone(), None);

    drop(alice);

    assert_eq!(bob.upgrade(), None);
}
