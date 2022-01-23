use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Human {
    alice: Rc<RefCell<String>>,
    bob: Weak<RefCell<String>>,
    mallory: Rc<RefCell<String>>,
}

fn main() {
    let alice: Rc<RefCell<String>>;
    let bob: Weak<RefCell<String>>;
    let mallory: Rc<RefCell<String>>;

    let s1 = String::from("hello");
    alice = Rc::new(RefCell::new(s1));
    bob = Rc::downgrade(&alice);
    mallory = Rc::clone(&alice);

    let mut h = Human { alice, bob, mallory };

    // The sum of the strong counts is 2.
    assert_eq!(2, Rc::strong_count(&h.alice));
    assert_eq!(1, Rc::weak_count(&h.alice));

    // Update value.
    h.mallory.borrow_mut().push_str(" world!");
    let s2 = String::from("hello world!");
    assert_eq!(s2, h.alice.borrow().clone());
    assert_eq!(s2, h.bob.upgrade().unwrap().borrow().clone());
    assert_eq!(s2, h.mallory.borrow().clone());
    println!("alice={:?}, bob={:?}", h.alice.borrow(), h.bob.upgrade());

    {
        // `try_unwrap()` fails because the strong count is greater than 1.
        let some_alice = Rc::try_unwrap(h.alice).unwrap_err();
        assert_eq!(
            Rc::new(RefCell::new(s2.clone())), some_alice
        );
        assert_eq!(2, Rc::strong_count(&h.mallory));

        // Drop `alice` trapped in try_unwrap to decrement a strong count.
        drop(some_alice);
        assert_eq!(1, Rc::strong_count(&h.mallory));

        // Return the strong count to 2 for testing.
        h.alice = Rc::clone(&h.mallory);
        assert_eq!(2, Rc::strong_count(&h.alice));
    }

    // Drop mallory to decrement one strong count.
    drop(h.mallory);

    // The strong count is 1 now, so try `try_unwrap`.
    assert_eq!(1, Rc::strong_count(&h.alice));
    assert_eq!(
        RefCell::new(s2),
        Rc::try_unwrap(h.alice).unwrap()
    );
    assert_eq!(None, h.bob.upgrade());
}
