use std::rc::{Rc, Weak};
use std::cell::{RefCell};
use std::fmt::{self, Debug};

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

// impl<T: Debug> Drop for DListNode<T> {
//     fn drop(&mut self) {
//         println!("> Dropping: DListNode {:?}", self.value);
//     }
// }

impl<T: Debug> fmt::Debug for DListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.prev.upgrade(), self.next.borrow().as_ref()) {
            (None, None) => {
                write!(
                    f, "(value:{:?}, prev:Nil, next:Nil)",
                    self.value.borrow().as_ref().unwrap()
                )
            },
            (Some(ref prev_rc_ref), None) => {
                write!(
                    f, "(value:{:?}, prev:{:?}, next:Nil)",
                    self.value.borrow().as_ref().unwrap(),
                    prev_rc_ref.borrow().as_ref().unwrap().value.borrow().as_ref().unwrap(),
                )
            },
            (None, Some(next)) => {
                write!(
                    f, "(value:{:?}, prev:Nil, next:{:?}) -> {:?}",
                    self.value.borrow().as_ref().unwrap(),
                    next.value.borrow().as_ref().unwrap(),
                    next,
                )
            },
            (Some(ref prev_rc_ref), Some(next)) => {
                write!(
                    f, "(value:{:?}, prev:{:?}, next:{:?}) -> {:?}",
                    self.value.borrow().as_ref().unwrap(),
                    prev_rc_ref.borrow().as_ref().unwrap().value.borrow().as_ref().unwrap(),
                    next.value.borrow().as_ref().unwrap(),
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
        let head: Rc<RefCell<Option<DListNode<T>>>>;
        head = Rc::new(RefCell::new(None));
        self.head.swap(&head);
        let node: DListNode<T> = match Rc::try_unwrap(head) {
            Ok(head_cell) => {
                head_cell.into_inner().unwrap()
            }
            Err(_head_rc) => return None,
        };
        let value: Option<T> = node.value.replace(None);

        let next = Rc::clone(&node.next);
        drop(node);
        if next.borrow().is_none() {
            // [ head ] -> [ node ] -> Nil
            //               ↑pop
            // [ head ] -> Nil
            self.head.replace(None);
            return value;
        }

        let mut node: DListNode<T> = match Rc::try_unwrap(next) {
            Ok(head_cell) => {
                head_cell.into_inner().unwrap()
            }
            Err(_head_rc) => return value,
        };
        let _ = std::mem::replace(&mut node.prev, Weak::new());

        self.head.replace(Some(node));
        value
    }

    /// # Examples
    ///
    /// ```
    /// use dlist_rc_refcell_opt::DList;
    /// let mut list: DList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.borrow().is_none() {
            return None;
        }

        let mut cur: Rc<RefCell<Option<DListNode<T>>>>;
        cur = Rc::clone(&self.head);

        while let Some(cur_node_ref) = Rc::clone(&cur).borrow().as_ref() {
            if cur_node_ref.next.borrow().is_none() {
                break;
            }
            cur = Rc::clone(&cur_node_ref.next);
        }

        // Update to None to the next pointer on the previous node.
        let last = cur;

        let last_prev_weak = Weak::clone(
            &last.borrow().as_ref().unwrap().prev
        );

        if last_prev_weak.upgrade().is_some() {
            let last_prev_rc = Rc::clone(
                last_prev_weak.upgrade().as_ref().unwrap()
            );

            let some_last_prev = last_prev_rc.replace(None);
            if let Some(last_prev_node) = some_last_prev {
                drop(last_prev_node.next);
                last_prev_rc.replace(
                    Some(DListNode {
                        value: last_prev_node.value,
                        next: Rc::new(RefCell::new(None)),
                        prev: last_prev_node.prev,
                    })
                );
            }
        } else {
            let some_last_prev = self.head.replace(None);
            if let Some(last_prev_node) = some_last_prev {
                drop(last_prev_node.next);
                return last_prev_node.value.borrow().clone();
            }
            return None;
        }

        assert_eq!(1, Rc::strong_count(&last));
        match Rc::try_unwrap(last) {
            Ok(last_cell) => {
                last_cell.into_inner().map(
                    |node| node.value.borrow().clone()
                ).unwrap()
            }
            Err(_last_rc) => None,
        }
    }
}

impl<T: Debug> Drop for DList<T> {
    fn drop(&mut self) {
        println!("> Dropping: DList");
    }
}

impl<T: Debug> fmt::Debug for DList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head.borrow().as_ref() {
            None => write!(f, "DList[]"),
            Some(ref head) => {
                write!(f, "DList[{:?}]", head)
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
            self.cur = cur_node_ref.next.borrow().as_ref().map(
                |_next_node_ref| Rc::downgrade(&Rc::clone(&cur_node_ref.next))
            );
        }
        cur_val
    }
}

#[cfg(test)]
mod tests;
