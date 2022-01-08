use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ListNode<T> {
    value: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

pub struct SinglyLinkedList<T> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: fmt::Debug> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.next {
            None => write!(f, "ListNode({:?},Nil)", self.value),
            Some(ref next) => {
                write!(f, "ListNode({:?},{})", self.value, next.borrow())
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            None => write!(f, "SinglyLinkedList(Nil)"),
            Some(ref head) => {
                write!(f, "SinglyLinkedList({})", head.borrow())
            }
        }
    }
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
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

        while let Some(ref next) = Rc::clone(&cur).borrow().next {
            cur = Rc::clone(next);
        }

        cur.borrow_mut().next = Some(
            Rc::new(RefCell::new(node_new))
        );
    }
}
