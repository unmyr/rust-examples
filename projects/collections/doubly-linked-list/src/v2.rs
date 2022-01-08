use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;

pub struct Node<T: Debug> {
    value: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Default)]
pub struct List<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug> Node<T> {
    pub fn new(v: T) -> Node<T> {
        Node { value: v, next: None, prev: None }
    }
}

impl<T: Debug> List<T> {
    pub fn push_back(&mut self, v: T) {
        let mut node_new = Node::new(v);
        let mut cur: Rc<RefCell<Node<T>>>;
        if let Some(ref head) = self.head {
            cur = Rc::clone(head);
        } else {
            self.head = Some(Rc::new(RefCell::new(node_new)));
            return;
        };

        while let Some(ref next) = Rc::clone(&cur).borrow().next {
            cur = Rc::clone(next);
        }
        node_new.prev = Some(Rc::downgrade(&cur));

        cur.borrow_mut().next = Some(
            Rc::new(RefCell::new(node_new))
        );
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut cur: Rc<RefCell<Node<T>>>;
        if let Some(ref head) = self.head {
            cur = Rc::clone(head);
        } else {
            return None;
        };

        while let Some(ref next) = Rc::clone(&cur).borrow().next {
            cur = Rc::clone(next);
        }

        if let Some(prev) = &Rc::clone(&cur).borrow_mut().prev {
            prev.upgrade().unwrap().borrow_mut().next = None;
        } else {
            self.head = None;
        }

        assert_eq!(Rc::strong_count(&cur), 1);
        assert_eq!(Rc::weak_count(&cur), 0);
        let last: Node<T> = Rc::try_unwrap(cur).ok().unwrap().into_inner();
        Some(last.value)
    }
}

impl<T: Debug> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.prev.as_ref(), self.next.as_ref()) {
            (None, None) => {
                write!(f, "Node({:?}, Nil, Nil)", self.value)
            },
            (Some(prev), None) => {
                write!(
                    f, "Node({:?}, {:?}, Nil)",
                    self.value,
                    Rc::clone(&prev.upgrade().unwrap()).borrow().value
                )
            },
            (None, Some(next)) => {
                write!(
                    f, "Node({:?}, Nil, {:?}), {}",
                    self.value, next.borrow().value, next.borrow()
                )
            },
            (Some(prev), Some(next)) => {
                write!(
                    f, "Node({:?}, {:?}, {:?}), {}",
                    self.value,
                    Rc::clone(&prev.upgrade().unwrap()).borrow().value,
                    next.borrow().value,
                    next.borrow()
                )
            }
        }
    }
}

impl<T: Debug> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            None => write!(f, "List[]"),
            Some(ref head) => {
                write!(f, "List[{}]", head.borrow())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::List;

    #[test]
    fn test_push_back_u8() {
        let mut list: List<u8> = Default::default();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}
