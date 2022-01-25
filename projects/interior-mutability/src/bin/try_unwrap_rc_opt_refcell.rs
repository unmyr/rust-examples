use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Human {
    alice: Rc<Option<RefCell<String>>>,
    bob: Weak<Option<RefCell<String>>>,
    mallory: Rc<Option<RefCell<String>>>,
}

fn main() {
    let alice: Rc<Option<RefCell<String>>>;
    let bob: Weak<Option<RefCell<String>>>;
    let mallory: Rc<Option<RefCell<String>>>;

    let s1 = String::from("hello");
    alice = Rc::new(Some(RefCell::new(s1)));
    bob = Rc::downgrade(&alice);
    mallory = Rc::clone(&alice);

    let mut h = Human { alice, bob, mallory };

    // The sum of the strong counts is 2.
    assert_eq!(2, Rc::strong_count(&h.alice));
    assert_eq!(1, Rc::weak_count(&h.alice));

    // Update value.
    if let Some(refcell) = h.mallory.as_ref() {
        refcell.borrow_mut().push_str(" world!");
    }
    let s2 = String::from("hello world!");
    assert_eq!(
        s2,
        h.alice.as_ref().as_ref().unwrap().borrow().clone()
    );
    assert_eq!(
        s2,
        h.bob.upgrade().unwrap().as_ref().as_ref().unwrap().borrow().clone()
    );
    assert_eq!(
        s2,
        h.mallory.as_ref().as_ref().unwrap().borrow().clone()
    );
    println!(
        "alice={:?}, bob={:?}",
        h.alice.as_ref().as_ref().unwrap().borrow(),
        h.bob.upgrade().unwrap().as_ref().as_ref().unwrap().borrow()
    );

    {
        // `try_unwrap()` fails because the strong count is greater than 1.
        let alice_rc = Rc::try_unwrap(h.alice).unwrap_err();
        assert_eq!(
            Rc::new(Some(RefCell::new(s2.clone()))), alice_rc
        );
        assert_eq!(2, Rc::strong_count(&h.mallory));

        // Drop `alice` trapped in try_unwrap to decrement a strong count.
        drop(alice_rc);
        assert_eq!(1, Rc::strong_count(&h.mallory));

        // Return the strong count to 2 for testing.
        h.alice = Rc::clone(&h.mallory);
        assert_eq!(2, Rc::strong_count(&h.alice));
    }

    // Drop mallory to decrement one strong count.
    std::mem::swap(&mut h.mallory, &mut Rc::new(None));

    // The strong count is 1 now, so try `try_unwrap`.
    assert_eq!(1, Rc::strong_count(&h.alice));
    let alice_rc = Rc::try_unwrap(h.alice).unwrap();
    assert_eq!(s2, alice_rc.unwrap().borrow().clone());
}