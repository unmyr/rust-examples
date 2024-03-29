use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut};
use std::fmt::{self, Debug};

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

// impl<T: Debug> Drop for DListNode<T> {
//     fn drop(&mut self) {
//         println!("> Dropping: DListNode {:?}", self.value);
//     }
// }

impl<T: Debug> fmt::Debug for DListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.prev.upgrade(), self.next.as_ref()) {
            (None, None) => {
                write!(f, "({:?}, prev:Nil, next:Nil)", self.value.borrow().as_ref().unwrap())
            },
            (Some(ref prev_rc_ref), None) => {
                match prev_rc_ref.as_ref() {
                    Some(prev_cell_ref) => {
                        write!(
                            f, "(value:{:?}, prev:{:?}, next:Nil)",
                            self.value.borrow().as_ref().unwrap(),
                            prev_cell_ref.borrow().value.borrow().as_ref().unwrap()
                        )
                    },
                    None => {
                        write!(
                            f, "(value:{:?}, prev:Nil, next:Nil)",
                            self.value.borrow().as_ref().unwrap()
                        )
                    }
                }

            },
            (None, Some(next)) => {
                write!(
                    f, "(value:{:?}, prev:Nil, next:{:?}) -> {:?}",
                    self.value.borrow().as_ref().unwrap(),
                    next.borrow().value.borrow().as_ref().unwrap(),
                    next.borrow()
                )
            },
            (Some(ref prev_rc_ref), Some(next)) => {
                match prev_rc_ref.as_ref() {
                    Some(prev_cell_ref) => {
                        write!(
                            f, "(value:{:?}, prev:{:?}, next:{:?}) -> {:?}",
                            self.value.borrow().as_ref().unwrap(),
                            prev_cell_ref.borrow().value.borrow().as_ref().unwrap(),
                            next.borrow().value.borrow().as_ref().unwrap(),
                            next.borrow()
                        )
                    },
                    None => {
                        write!(
                            f, "(value:{:?}, prev:Nil, next:{:?}) -> {:?}",
                            self.value.borrow().as_ref().unwrap(),
                            next.borrow().value.borrow().as_ref().unwrap(),
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

impl<T: Debug> DList<T> {
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
            let mut next_rc = Rc::clone(&cur_node.borrow().next);
            assert_eq!(Rc::strong_count(&next_rc), 2);
            unsafe {
                let ptr = Rc::into_raw(next_rc);
                Rc::decrement_strong_count(ptr);
                next_rc = Rc::from_raw(ptr);
            }
            assert_eq!(Rc::strong_count(&next_rc), 1);
            if let Some(cur_opt) = Rc::get_mut(&mut next_rc) {
                Option::<RefCell<DListNode<T>>>::replace(
                    cur_opt,
                    RefCell::new(node_new)
                );
            } else {
                println!("Failed.");
            }
            unsafe {
                let ptr = Rc::into_raw(next_rc);
                Rc::increment_strong_count(ptr);
            }
        }
        drop(cur);
    }

    pub fn push_front(&mut self, _v: T) {}

    /// # Examples
    ///
    /// ```
    /// use dlist_rc_opt_refcell::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let head: Rc<Option<RefCell<DListNode<T>>>>;
        head = Rc::new(None);

        let mut old_head: Rc<_> = std::mem::replace(&mut self.head, head);

        if Rc::strong_count(&old_head) == 2 {
            unsafe {
                let ptr = Rc::into_raw(old_head);
                Rc::decrement_strong_count(ptr);
                old_head = Rc::from_raw(ptr);
            }
        }

        let node: DListNode<T> = match Rc::try_unwrap(old_head) {
            Ok(some_refcell) => some_refcell.unwrap().into_inner(),
            Err(_rc) => {
                return None
            },
        };
        let value: Option<T> = node.value.into_inner();

        let _ = std::mem::replace(
            &mut self.head, node.next
        );

        value
    }
}


impl<T: Clone + Debug> DList<T> {
    /// # Examples
    ///
    /// ```
    /// use dlist_rc_opt_refcell::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// // assert_eq!(list.pop_back(), Some(2));
    /// // assert_eq!(list.pop_back(), Some(1));
    /// // assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let mut cur: Rc<Option<RefCell<DListNode<T>>>>;
        cur = Rc::clone(&self.head);

        while let Some(cur_node) = Rc::clone(&cur).as_ref() {
            if cur_node.borrow().next.is_none() {
                break;
            }
            cur = Rc::clone(&cur_node.borrow().next);
        }

        let tail_rc = cur;

        // Update to None to the next pointer on the previous node.
        let prev_weak = Weak::clone(
            &(tail_rc.as_ref().as_ref().unwrap().borrow().prev)
        );

        if let Some(prev_rc) = prev_weak.upgrade() {
            RefMut::map(
                prev_rc.as_ref().as_ref().unwrap().borrow_mut(),
                |v| {
                    v.next = Rc::new(None);
                    v
                }
            );
        } else {
            self.head = Rc::new(None);
        }

        let last_cell_ref = tail_rc.as_ref().as_ref().unwrap();
        let value_cell = last_cell_ref.borrow().value.clone();
        value_cell.into_inner()
    }
}

impl<T: Debug> Drop for DList<T> {
    fn drop(&mut self) {
        println!("> Dropping: DList");
    }
}

impl<T: Debug> fmt::Debug for DList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head.as_ref() {
            None => write!(f, "DList[]"),
            Some(ref head) => {
                write!(f, "DList[{:?}]", head.borrow())
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
            self.cur = cur_cell.borrow().next.as_ref().as_ref().map(
                |_next_cell| Rc::downgrade(&Rc::clone(&cur_cell.borrow().next))
            );
        }
        cur_val
    }
}

#[cfg(test)]
mod tests;
