use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ListNode<T> {
    value: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
}

impl<T: fmt::Debug> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.next {
            Some(ref next) => {
                write!(f, "ListNode({:?}), {}", self.value, next.borrow())
            },
            None => write!(f, "ListNode({:?})", self.value)
        }
    }
}

#[derive(Default)]
pub struct SinglyLinkedList<T> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            head: None,
        }
    }

    pub fn push_back(&mut self, v: T) {
        let node_new = ListNode::new(v);
        let mut cur: Rc<RefCell<ListNode<T>>>;
        if let Some(ref head) = self.head {
            cur = Rc::clone(head);
        } else {
            self.head = Some(Rc::new(RefCell::new(node_new)));
            return;
        };

        loop {
            if let Some(ref next) = Rc::clone(&cur).borrow().next {
                cur = Rc::clone(next);
                continue;
            }

            cur.borrow_mut().next = Some(
                Rc::new(RefCell::new(node_new))
            );
            return;
        }
    }
}

impl<T: fmt::Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            Some(ref head) => {
                write!(f, "SinglyLinkedList[{}]", head.borrow())
            }
            None => write!(f, "SinglyLinkedList[]")
        }
    }
}
