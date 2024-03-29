use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{self, Debug};

pub struct DListNode<T: Debug> {
    value: T,
    prev: Option<Weak<RefCell<DListNode<T>>>>,
    next: Option<Rc<RefCell<DListNode<T>>>>,
}

impl<T: Debug> DListNode<T> {
    pub fn new(v: T) -> DListNode<T> {
        DListNode { value: v, next: None, prev: None }
    }
}

impl<T: Debug> Drop for DListNode<T> {
    fn drop(&mut self) {
        println!("> Dropping: DListNode {:?}", self.value);
    }
}

impl<T: Debug> fmt::Display for DListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.prev.as_ref(), self.next.as_ref()) {
            (None, None) => {
                write!(f, "DListNode({:?}, Nil, Nil)", self.value)
            },
            (Some(prev), None) => {
                write!(
                    f, "DListNode({:?}, {:?}, Nil)",
                    self.value,
                    Rc::clone(&prev.upgrade().unwrap()).borrow().value
                )
            },
            (None, Some(next)) => {
                write!(
                    f, "DListNode({:?}, Nil, {:?}), {}",
                    self.value, next.borrow().value, next.borrow()
                )
            },
            (Some(prev), Some(next)) => {
                write!(
                    f, "DListNode({:?}, {:?}, {:?}), {}",
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
pub struct DList<T: Debug> {
    head: Option<Rc<RefCell<DListNode<T>>>>,
}

impl<T: Debug> DList<T> {
    /// # Examples
    ///
    /// ```
    /// use dlist_opt_rc_refcell::v1::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, v: T) {
        let mut node_new = DListNode::new(v);
        let mut cur: Rc<RefCell<DListNode<T>>>;
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
}

impl<T: Debug + Clone> DList<T> {
    /// # Examples
    ///
    /// ```
    /// use dlist_opt_rc_refcell::v1::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        let head = match self.head {
            Some(ref head) => Rc::clone(head),
            None => return None,
        };
        assert_eq!(Rc::strong_count(&head), 2);
        self.head = None;
        assert_eq!(Rc::strong_count(&head), 1);
        let mut node: DListNode<T> = Rc::try_unwrap(head).ok().unwrap().into_inner();
        if let Some(ref next) = node.next {
            if let Some(ref prev) = next.borrow().prev {
                // The previous node has already moved.
                assert!(prev.upgrade().is_none());
            }
            next.borrow_mut().prev = None;
        }
        self.head = node.next.take();
        Some(node.value.clone())
    }

    /// # Examples
    ///
    /// ```
    /// use dlist_opt_rc_refcell::v1::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        let mut cur: Rc<RefCell<DListNode<T>>>;
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
        let last: DListNode<T> = Rc::try_unwrap(cur).ok().unwrap().into_inner();
        Some(last.value.clone())
    }
}

impl<T: Debug> Drop for DList<T> {
    fn drop(&mut self) {
        println!("> Dropping: DList");
    }
}

impl<T: Debug> fmt::Display for DList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            None => write!(f, "DList[]"),
            Some(ref head) => {
                write!(f, "DList[{}]", head.borrow())
            }
        }
    }
}

pub struct DListIterator<T: Debug> {
    cur: Option<Weak<RefCell<DListNode<T>>>>
}

impl<T: Debug> DList<T> {
    /// # Examples
    ///
    /// ```
    /// use dlist_opt_rc_refcell::v1::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> DListIterator<T> {
        if let Some(ref head) = self.head {
            DListIterator {
                cur: Some(Rc::downgrade(&Rc::clone(head)))
            }
        } else {
            DListIterator { cur: None }
        }
    }
}

impl<T: Clone + Debug> Iterator for DListIterator<T> {
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

        let cur_val = cur_strong.borrow().value.clone();
        if let Some(ref next) = cur_strong.borrow().next {
            self.cur = Some(Rc::downgrade(next))
        } else {
            self.cur = None;
        }
        Some(cur_val)
    }
}

#[cfg(test)]
mod tests;
