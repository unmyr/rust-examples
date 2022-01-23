use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let alice: Rc<RefCell<String>>;
    let bob: Weak<RefCell<String>>;
    let mallory: Rc<RefCell<String>>;
    let s1 = String::from("hello");

    // The sum of the strong counts is 2.
    alice = Rc::new(RefCell::new(s1));
    bob = Rc::downgrade(&alice);
    mallory = Rc::clone(&alice);
    assert_eq!(2, Rc::strong_count(&alice));
    assert_eq!(1, Rc::weak_count(&alice));

    // Update value.
    mallory.borrow_mut().push_str(" world!");
    let s2 = String::from("hello world!");
    assert_eq!(s2, alice.borrow().clone());
    assert_eq!(s2, bob.upgrade().unwrap().borrow().clone());
    assert_eq!(s2, mallory.borrow().clone());
    println!("alice={:?}, bob={:?}", alice.borrow(), bob.upgrade());

    // Drop mallory to decrement one strong count.
    drop(mallory);

    // The strong count is 1 now, so try `try_unwrap`.
    assert_eq!(1, Rc::strong_count(&alice));
    assert_eq!(
        RefCell::new(s2),
        Rc::try_unwrap(alice).unwrap()
    );
    assert_eq!(None, bob.upgrade());
}
