use std::fmt::{self};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct ListNode {
    value: String,
    next: Rc<Option<RefCell<ListNode>>>,
    prev: Weak<Option<RefCell<ListNode>>>,
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let some_prev = Weak::upgrade(&self.prev);
        let some_next = self.next.as_ref();
        match (some_prev, some_next) {
            (None, None) => {
                write!(f, "ListNode({:?}, prev:Nil, next:Nil)", self.value)
            },
            (Some(some_prev_cell_ref), None) => {
                write!(
                    f, "ListNode(value:{:?}, prev:{:?}, next:Nil)",
                    self.value,
                    match some_prev_cell_ref.as_ref() {
                        Some(prev_cell_ref) => prev_cell_ref.borrow().value.clone(),
                        None => String::from("Nil")
                    },
                )
            },
            (None, Some(next)) => {
                write!(
                    f, "ListNode(value:{:?}, prev:Nil, next:{:?}), {:?}",
                    self.value,
                    next.borrow().value,
                    next.borrow(),
                )
            },
            (Some(some_prev_cell_ref), Some(next)) => {
                write!(
                    f, "ListNode(value:{:?}, prev:{:?}, next:{:?}), {:?}",
                    self.value,
                    match some_prev_cell_ref.as_ref() {
                        Some(prev_cell_ref) => prev_cell_ref.borrow().value.clone(),
                        None => String::from("Nil")
                    },
                    next.borrow().value,
                    next.borrow(),
                )
            }
        }
    }
}

fn main() {
    let node_3 = ListNode {
        value: String::from("node_3"),
        next: Rc::new(None),
        prev: Weak::new(),
    };
    let node_2 = ListNode {
        value: String::from("node_2"),
        next: Rc::new(Some(RefCell::new(node_3))),
        prev: Weak::new(),
    };
    let node_1 = ListNode {
        value: String::from("node_1"),
        next: Rc::new(Some(RefCell::new(node_2))),
        prev: Weak::new(),
    };

    let node_1_rc: Rc<Option<RefCell<ListNode>>>;
    let node_2_rc: Rc<Option<RefCell<ListNode>>>;
    let node_3_rc: Rc<Option<RefCell<ListNode>>>;

    node_1_rc = Rc::new(Some(RefCell::new(node_1)));
    node_2_rc = match node_1_rc.as_ref() {
        Some(node_1_cell_ref) => Rc::clone(&node_1_cell_ref.borrow().next),
        None => Rc::new(None),
    };
    node_3_rc = match node_2_rc.as_ref() {
        Some(node_2_cell_ref) => Rc::clone(&node_2_cell_ref.borrow().next),
        None => Rc::new(None),
    };

    if let Some(node_3_ref_cell) = node_3_rc.as_ref() {
        node_3_ref_cell.borrow_mut().prev = Rc::downgrade(&node_2_rc);
    };
    if let Some(node_2_ref_cell) = node_2_rc.as_ref() {
        node_2_ref_cell.borrow_mut().prev = Rc::downgrade(&node_1_rc);
    };
    assert_eq!(1, Rc::strong_count(&node_1_rc));
    assert_eq!(2, Rc::strong_count(&node_2_rc));
    assert_eq!(2, Rc::strong_count(&node_3_rc));

    dbg!(&node_1_rc);
}