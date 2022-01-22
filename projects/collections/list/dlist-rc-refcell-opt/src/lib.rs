use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct DListNode<T: Debug> {
    value: RefCell<Option<T>>,
    prev: Weak<RefCell<Option<DListNode<T>>>>,
    next: Rc<RefCell<Option<DListNode<T>>>>,
}

impl<T: Debug> DListNode<T> {
    pub fn new(v: T) -> DListNode<T> {
        DListNode {
            value: RefCell::new(Some(v)),
            next: Rc::new(RefCell::new(None)),
            prev: Weak::new(),
        }
    }
}

impl<T: Debug> Drop for DListNode<T> {
    fn drop(&mut self) {
        println!("> Dropping: DListNode {:?}", self.value);
    }
}

impl<T: Debug> fmt::Display for DListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.prev.upgrade(), self.next.borrow().as_ref()) {
            (None, None) => {
                write!(f, "DListNode({:?}, Nil, Nil)", self.value.borrow())
            },
            (Some(ref prev_rc_ref), None) => {
                write!(
                    f, "DListNode(value:{:?}, prev:{:?}, Nil)",
                    self.value.borrow(),
                    prev_rc_ref.borrow().as_ref().unwrap().value,
                )
            },
            (None, Some(next)) => {
                write!(
                    f, "DListNode({:?}, Nil, {:?}), {}",
                    self.value.borrow(),
                    next.value.borrow(),
                    next,
                )
            },
            (Some(ref prev_rc_ref), Some(next)) => {
                write!(
                    f, "DListNode({:?}, {:?}, {:?}), {}",
                    self.value.borrow(),
                    prev_rc_ref.borrow().as_ref().unwrap().value,
                    next.value.borrow(),
                    next,
                )
            }
        }
    }
}

#[derive(Default)]
pub struct DList<T: Debug> {
    head: Rc<RefCell<Option<DListNode<T>>>>
}

impl<T: Clone + Debug> DList<T> {
    /// # Examples
    ///
    /// ```
    /// use dlist_rc_refcell_opt::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, v: T) {
        let mut node_new = DListNode::new(v);
        if self.head.borrow().is_none() {
            self.head = Rc::new(RefCell::new(Some(node_new)));
            return;
        }
        let mut cur: Rc<RefCell<Option<DListNode<T>>>>;
        cur = Rc::clone(&self.head);

        while let Some(cur_node_ref) = Rc::clone(&cur).borrow().as_ref() {
            if cur_node_ref.next.borrow().is_none() {
                break;
            }
            cur = Rc::clone(&cur_node_ref.next);
        }
        node_new.prev = Rc::downgrade(&cur);

        if let Some(cur_node_ref) = Rc::clone(&cur).borrow().as_ref() {
            Rc::clone(&cur_node_ref.next).replace(Some(node_new));
        }
        drop(cur);
        dbg!(Rc::strong_count(&self.head));
        dbg!(Rc::weak_count(&self.head));
    }

    /// # Examples
    ///
    /// ```
    /// use dlist_rc_refcell_opt::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&self) -> Option<T> {
        if self.head.borrow().is_none() {
            return None;
        }
        let value = self.head.borrow().as_ref().unwrap().value.clone().into_inner();

        let head_next = Rc::clone(&self.head.borrow().as_ref().unwrap().next);
        self.head.swap(&head_next);

        value
    }

    pub fn pop_back(&mut self) -> Option<T> {
        None
    }
}

impl<T: Debug> Drop for DList<T> {
    fn drop(&mut self) {
        println!("> Dropping: DList");
    }
}

impl<T: Debug> fmt::Display for DList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head.borrow().as_ref() {
            None => write!(f, "DList[]"),
            Some(ref head) => {
                write!(f, "DList[{}]", head)
            }
        }
    }
}

pub struct DListIterator<T: Debug> {
    cur: Option<Weak<RefCell<Option<DListNode<T>>>>>
}

impl<T: Debug> DList<T> {
    /// # Examples
    ///
    /// ```
    /// use dlist_rc_refcell_opt::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> DListIterator<T> {
        if self.head.borrow().is_none() {
            DListIterator { cur: None }
        } else {
            DListIterator {
                cur: Some(
                    Rc::downgrade(&Rc::clone(&self.head))
                )
            }
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

        let cur_val: Option<T>;
        cur_val = match cur_strong.borrow().as_ref() {
            None => return None,
            Some(cur_node_ref) => cur_node_ref.value.clone().into_inner(),
        };

        if let Some(cur_node_ref) = cur_strong.borrow().as_ref() {
            self.cur = match cur_node_ref.next.borrow().as_ref() {
                Some(_next_node_ref) => {
                    Some(Rc::downgrade(&Rc::clone(&cur_node_ref.next)))
                },
                None => None,
            }
        }
        cur_val
    }
}

// #[cfg(test)]
// mod tests;
