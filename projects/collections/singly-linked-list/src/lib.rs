use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SinglyLinkedList<T> {
    value: T,
    next: Option<Rc<RefCell<SinglyLinkedList<T>>>>,
}

impl<T: std::fmt::Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.next {
            None => write!(f, "SinglyLinkedList({:?}, Nil)", self.value),
            Some(ref next) => {
                write!(f, "SinglyLinkedList({:?}, {})", self.value, next.borrow())
            }
        }
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new(v: T) -> SinglyLinkedList<T> {
        SinglyLinkedList {
            value: v,
            next: None,
        }
    }
    pub fn push_back(&mut self, v: T) {
        let node_new = SinglyLinkedList::new(v);

        let mut cur: Option<Rc<RefCell<SinglyLinkedList<T>>>>;
        if let Some(ref next) = self.next {
            cur = Some(Rc::clone(next));
        } else {
            self.next = Some(Rc::new(RefCell::new(node_new)));
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
