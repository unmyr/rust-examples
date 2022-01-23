use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let alice: Option<Rc<RefCell<String>>>;
    let bob: Option<Weak<RefCell<String>>>;
    let mut mallory: Option<Rc<RefCell<String>>>;
    let s1 = String::from("hello");

    // The sum of the strong counts is 2.
    alice = Some(Rc::new(RefCell::new(s1)));
    bob = Some(Rc::downgrade(alice.as_ref().unwrap()));
    mallory = Some(Rc::clone(alice.as_ref().unwrap()));
    assert_eq!(2, Rc::strong_count(alice.as_ref().unwrap()));
    assert_eq!(1, Rc::weak_count(alice.as_ref().unwrap()));

    // Update value.
    mallory.as_ref().unwrap().borrow_mut().push_str(" world!");
    let s2 = String::from("hello world!");
    assert_eq!(s2, alice.as_ref().unwrap().borrow().clone());
    assert_eq!(s2, bob.as_ref().unwrap().upgrade().unwrap().borrow().clone());
    assert_eq!(s2, mallory.as_ref().unwrap().borrow().clone());
    println!(
        "alice={:?}, bob={:?}",
        alice.as_ref().unwrap().borrow(),
        bob.as_ref().unwrap().upgrade()
    );

    // `try_unwrap()` fails because the strong count is greater than 1.
    let some_alice = alice.map(
        |alice_rc| Rc::try_unwrap(alice_rc).unwrap_err()
    );
    assert_eq!(
        Some(Rc::new(RefCell::new(s2.clone()))), some_alice
    );
    assert_eq!(2, Rc::strong_count(mallory.as_ref().unwrap()));

    // Drop `alice` trapped in try_unwrap to decrement a strong count.
    drop(some_alice);
    assert_eq!(1, Rc::strong_count(mallory.as_ref().unwrap()));

    // Return the strong count to 2 for testing.
    let alice = Some(Rc::clone(mallory.as_ref().unwrap()));
    assert_eq!(2, Rc::strong_count(alice.as_ref().unwrap()));

    // Drop mallory to decrement one strong count.
    mallory = None;
    assert_eq!(None, mallory);

    // The strong count is 1 now, so try `try_unwrap`.
    assert_eq!(1, Rc::strong_count(alice.as_ref().unwrap()));
    let some_alice = alice.map(
        |alice_rc| Rc::try_unwrap(alice_rc).unwrap()
    );
    assert_eq!(Some(RefCell::new(s2)), some_alice);
}