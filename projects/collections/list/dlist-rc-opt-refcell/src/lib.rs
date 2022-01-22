use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct DListNode<T: Debug> {
    value: RefCell<Option<T>>,
    prev: Weak<Option<RefCell<DListNode<T>>>>,
    next: Rc<Option<RefCell<DListNode<T>>>>,
}

impl<T: Debug> DListNode<T> {
    pub fn new(v: T) -> DListNode<T> {
        DListNode {
            value: RefCell::new(Some(v)),
            next: Rc::new(None),
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
        match (self.prev.upgrade(), self.next.as_ref()) {
            (None, None) => {
                write!(f, "DListNode({:?}, Nil, Nil)", self.value.borrow())
            },
            (Some(ref prev_rc_ref), None) => {
                match prev_rc_ref.as_ref() {
                    Some(prev_cell_ref) => {
                        write!(
                            f, "DListNode(value:{:?}, prev:{:?}, Nil)",
                            self.value.borrow(),
                            prev_cell_ref.borrow().value
                        )
                    },
                    None => {
                        write!(
                            f, "DListNode(value:{:?}, prev:Nil, Nil)",
                            self.value.borrow()
                        )
                    }
                }

            },
            (None, Some(next)) => {
                write!(
                    f, "DListNode({:?}, Nil, {:?}), {}",
                    self.value.borrow(),
                    next.borrow().value.borrow(),
                    next.borrow()
                )
            },
            (Some(ref prev_rc_ref), Some(next)) => {
                match prev_rc_ref.as_ref() {
                    Some(prev_cell_ref) => {
                        write!(
                            f, "DListNode({:?}, {:?}, {:?}), {}",
                            self.value.borrow(),
                            prev_cell_ref.borrow().value,
                            next.borrow().value.borrow(),
                            next.borrow()
                        )
                    },
                    None => {
                        write!(
                            f, "DListNode({:?}, Nil, {:?}), {}",
                            self.value.borrow(),
                            next.borrow().value.borrow(),
                            next.borrow()
                        )
                    },
                }

            }
        }
    }
}

#[derive(Default)]
pub struct DList<T: Debug> {
    head: Rc<Option<RefCell<DListNode<T>>>>
}

impl<T: Clone + Debug> DList<T> {
    /// # Examples
    ///
    /// ```
    /// use dlist_rc_opt_refcell::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, v: T) {
        let mut node_new = DListNode::new(v);
        if self.head.is_none() {
            self.head = Rc::new(Some(RefCell::new(node_new)));
            return;
        }
        let mut cur: Rc<Option<RefCell<DListNode<T>>>>;
        cur = Rc::clone(&self.head);

        while let Some(cur_node) = Rc::clone(&cur).as_ref() {
            if cur_node.borrow().next.is_none() {
                break;
            }
            cur = Rc::clone(&cur_node.borrow().next);
        }
        node_new.prev = Rc::downgrade(&cur);

        if let Some(cur_node) = Rc::clone(&cur).as_ref() {
            if let Some(next_cell_ref) =  Rc::clone(&cur_node.borrow().next).as_ref() {
                next_cell_ref.replace(node_new);
            }
        }
        drop(cur);
        dbg!(Rc::strong_count(&self.head));
        dbg!(Rc::weak_count(&self.head));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        None
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
        match self.head.as_ref() {
            None => write!(f, "DList[]"),
            Some(ref head) => {
                write!(f, "DList[{}]", head.borrow())
            }
        }
    }
}

pub struct DListIterator<T: Debug> {
    cur: Option<Weak<Option<RefCell<DListNode<T>>>>>
}

impl<T: Debug> DList<T> {
    /// # Examples
    ///
    /// ```
    /// use dlist_rc_opt_refcell::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> DListIterator<T> {
        if self.head.is_none() {
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
        cur_val = match cur_strong.as_ref() {
            None => return None,
            Some(cur_cell) => cur_cell.borrow().value.clone().into_inner(),
        };

        if let Some(cur_cell) = cur_strong.as_ref() {
            self.cur = match cur_cell.borrow().next.as_ref() {
                Some(_next_cell) => {
                    Some(Rc::downgrade(&Rc::clone(&cur_cell.borrow().next)))
                },
                None => None,
            }
        }
        cur_val
    }
}

// #[cfg(test)]
// mod tests;
