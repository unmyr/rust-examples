use std::default::Default;
use std::fmt;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct ListNode<T> {
    value: T,
    next: RefCell<Option<Rc<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: RefCell::new(None) }
    }
}

impl<T: fmt::Debug> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.next.borrow().as_ref() {
            Some(next) => {
                write!(f, "ListNode({:?}), {}", self.value, next)
            },
            None => write!(f, "ListNode({:?})", self.value)
        }
    }
}

#[derive(Default)]
pub struct SinglyLinkedList<T> {
    head: RefCell<Option<Rc<ListNode<T>>>>,
}

impl<T: fmt::Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head.borrow().as_ref() {
            Some(ref head) => {
                write!(f, "SinglyLinkedList[{}]", head)
            }
            None => write!(f, "SinglyLinkedList[]")
        }
    }
}

impl<T> SinglyLinkedList<T> {
    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_refcell_opt_rc::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, v: T) {
        let node_new = ListNode::new(v);
        let mut cur: Rc<ListNode<T>>;
        if self.head.borrow().as_ref().is_none() {
            self.head.replace(Some(Rc::new(node_new)));
            return;
        }
        cur = Rc::clone(&self.head.borrow().clone().unwrap());

        while let Some(next) = Rc::clone(&cur).next.borrow().as_ref() {
            cur = Rc::clone(next);
        }

        cur.next.replace(Some(Rc::new(node_new)));
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_refcell_opt_rc::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        let mut some_prev: Option<Rc<ListNode<T>>> = None;
        let mut cur: Rc<ListNode<T>>;
        if let Some(ref head) = self.head.borrow().as_ref() {
            cur = Rc::clone(head);
        } else {
            // You can't pop the head of the list.
            return None;
        };

        while let Some(next) = Rc::clone(&cur).next.borrow().as_ref() {
            some_prev = Some(Rc::clone(&cur));
            cur = Rc::clone(next);
        }

        if let Some(prev) = some_prev {
            prev.next.replace(None);
        } else {
            self.head.replace(None);
        }

        let last: ListNode<T> = Rc::try_unwrap(cur).ok().unwrap();
        Some(last.value)
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_refcell_opt_rc::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        let head = match self.head.borrow().as_ref() {
            Some(head) => Rc::clone(head),
            None => return None,
        };
        assert_eq!(Rc::strong_count(&head), 2);
        self.head = RefCell::new(None);
        assert_eq!(Rc::strong_count(&head), 1);
        let node: ListNode<T> = Rc::try_unwrap(head).ok().unwrap();
        self.head = node.next;
        Some(node.value)
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_refcell_opt_rc::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> SinglyLinkedListIterator<T> {
        if let Some(ref head) = self.head.borrow().as_ref() {
            SinglyLinkedListIterator {
                cur: RefCell::new(Some(Rc::downgrade(&Rc::clone(head))))
            }
        } else {
            SinglyLinkedListIterator { cur: RefCell::new(None) }
        }
    }
}

pub struct SinglyLinkedListIterator<T> {
    cur: RefCell<Option<Weak<ListNode<T>>>>
}

impl<T:Clone> Iterator for SinglyLinkedListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.borrow().as_ref().is_none() {
            return None;
        }
        let cur_weak = self.cur.borrow().clone().unwrap();

        let cur_strong = match cur_weak.upgrade() {
            Some(cur_strong) => cur_strong,
            None => return None,
        };

        let cur_val = cur_strong.value.clone();
        if let Some(ref next) = cur_strong.next.borrow().as_ref() {
            self.cur = RefCell::new(Some(Rc::downgrade(next)));
        } else {
            self.cur = RefCell::new(None);
        }
        Some(cur_val)
    }
}

#[cfg(test)]
mod tests;
