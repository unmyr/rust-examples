use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Human {
    alice: Option<Rc<RefCell<String>>>,
    bob: Option<Weak<RefCell<String>>>,
    mallory: Option<Rc<RefCell<String>>>,
}

fn main() {
    let alice: Option<Rc<RefCell<String>>>;
    let bob: Option<Weak<RefCell<String>>>;
    let mallory: Option<Rc<RefCell<String>>>;

    let s1 = String::from("hello");
    alice = Some(Rc::new(RefCell::new(s1)));
    bob = Some(Rc::downgrade(alice.as_ref().unwrap()));
    mallory = Some(Rc::clone(alice.as_ref().unwrap()));

    let mut h = Human { alice, bob, mallory };

    // The sum of the strong counts is 2.
    assert_eq!(2, Rc::strong_count(h.alice.as_ref().unwrap()));
    assert_eq!(1, Rc::weak_count(h.alice.as_ref().unwrap()));

    // Update value.
    h.mallory.as_ref().unwrap().borrow_mut().push_str(" world!");
    let s2 = String::from("hello world!");
    assert_eq!(s2, h.alice.as_ref().unwrap().borrow().clone());
    assert_eq!(s2, h.bob.as_ref().unwrap().upgrade().unwrap().borrow().clone());
    assert_eq!(s2, h.mallory.as_ref().unwrap().borrow().clone());
    println!(
        "alice={:?}, bob={:?}",
        h.alice.as_ref().unwrap().borrow(),
        h.bob.as_ref().unwrap().upgrade()
    );

    {
        // `try_unwrap()` fails because the strong count is greater than 1.
        let some_alice = h.alice.map(
            |alice_rc| Rc::try_unwrap(alice_rc).unwrap_err()
        );
        assert_eq!(
            Some(Rc::new(RefCell::new(s2.clone()))), some_alice
        );
        assert_eq!(2, Rc::strong_count(h.mallory.as_ref().unwrap()));

        // Drop `alice` trapped in try_unwrap to decrement a strong count.
        drop(some_alice);
        assert_eq!(1, Rc::strong_count(h.mallory.as_ref().unwrap()));

        // Return the strong count to 2 for testing.
        h.alice = Some(Rc::clone(h.mallory.as_ref().unwrap()));
        assert_eq!(2, Rc::strong_count(h.alice.as_ref().unwrap()));
    }

    // Drop mallory to decrement one strong count.
    h.mallory = None;
    assert_eq!(None, h.mallory);

    // The strong count is 1 now, so try `try_unwrap`.
    assert_eq!(1, Rc::strong_count(h.alice.as_ref().unwrap()));
    let some_alice = h.alice.map(
        |alice_rc| Rc::try_unwrap(alice_rc).unwrap()
    );
    assert_eq!(Some(RefCell::new(s2)), some_alice);
}