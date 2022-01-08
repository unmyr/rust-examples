use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ListNode<T: Debug> {
    value: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

#[derive(Default)]
pub struct SinglyLinkedList<T: Debug> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: Debug> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.next {
            None => write!(f, "ListNode({:?},Nil)", self.value),
            Some(ref next) => {
                write!(f, "ListNode({:?},{})", self.value, next.borrow())
            }
        }
    }
}

impl<T: Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            None => write!(f, "SinglyLinkedList(Nil)"),
            Some(ref head) => {
                write!(f, "SinglyLinkedList({})", head.borrow())
            }
        }
    }
}
impl<T: Debug> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            head: None,
        }
    }

    pub fn push_back(&mut self, v: T) {
        let node_new = ListNode::new(v);

        let mut cur: Option<Rc<RefCell<ListNode<T>>>>;
        if let Some(ref head) = self.head {
            cur = Some(Rc::clone(head));
        } else {
            self.head = Some(Rc::new(RefCell::new(node_new)));
            return;
        };

        loop {
            let cur_cloned = match cur {
                None => break,
                Some(ref n) => Rc::clone(n)
            };
            cur = match cur_cloned.borrow().next {
                Some(ref next) => Some(Rc::clone(next)),
                None => {
                    unsafe {
                        (*cur_cloned.as_ptr()).next = Some(
                            Rc::new(
                                RefCell::new(node_new)
                            )
                        );
                    }
                    return;
                }
            };
        }
    }
}
