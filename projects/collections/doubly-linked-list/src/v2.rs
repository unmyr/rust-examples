use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{self, Debug};

pub struct Node<T: Debug> {
    value: RefCell<Option<T>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug> Node<T> {
    pub fn new(v: T) -> Node<T> {
        Node { value: RefCell::new(Some(v)), next: None, prev: None }
    }
}

impl<T: Debug> Drop for Node<T> {
    fn drop(&mut self) {
        println!("> Dropping: Node {:?}", self.value);
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

#[derive(Default)]
pub struct List<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
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
        last.value.take()
    }

    /// # Examples
    ///
    /// ```
    /// use doubly_linked_list::v2::List;
    /// let mut list: List<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> ListIterator<T> {
        if let Some(ref head) = self.head {
            ListIterator {
                cur: Some(Rc::downgrade(&Rc::clone(head)))
            }
        } else {
            ListIterator { cur: None }
        }
    }
}

impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        println!("> Dropping: List");
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

pub struct ListIterator<T: Debug> {
    cur: Option<Weak<RefCell<Node<T>>>>
}

impl<T: Clone + Debug> Iterator for ListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let cur_weak = match self.cur {
            Some(ref cur_weak) => cur_weak,
            None => return None,
        };

        let cur_strong = match cur_weak.upgrade() {
            Some(cur_strong) => cur_strong,
            None => return None,
        };

        let cur_val = cur_strong.borrow().value.borrow().clone();
        if let Some(ref next) = cur_strong.borrow().next {
            self.cur = Some(Rc::downgrade(next))
        } else {
            self.cur = None;
        }
        cur_val
    }
}

#[cfg(test)]
mod tests {
    use super::List;

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
