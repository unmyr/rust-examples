use std::default::Default;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;


pub struct ListNode<T> {
    value: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

#[derive(Default)]
pub struct SinglyLinkedList<T> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: fmt::Debug> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.next {
            Some(ref next) => {
                write!(f, "ListNode({:?}), {}", self.value, next.borrow())
            },
            None => write!(f, "ListNode({:?})", self.value)
        }
    }
}

impl<T: fmt::Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            Some(ref head) => {
                write!(f, "SinglyLinkedList[{}]", head.borrow())
            }
            None => write!(f, "SinglyLinkedList[]")
        }
    }
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
}

pub struct SinglyLinkedListIterator<T> {
    cur: Option<Weak<RefCell<ListNode<T>>>>
}

impl<T> SinglyLinkedList<T> {
    /// # Examples
    ///
    /// ```
    /// use list::v6::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, v: T) {
        let node_new = ListNode::new(v);
        let mut cur: Rc<RefCell<ListNode<T>>>;
        if let Some(ref head) = self.head {
            cur = Rc::clone(head);
        } else {
            self.head = Some(Rc::new(RefCell::new(node_new)));
            return;
        };

        while let Some(ref next) = Rc::clone(&cur).borrow().next {
            cur = Rc::clone(next);
        }

        cur.borrow_mut().next = Some(
            Rc::new(RefCell::new(node_new))
        );
    }

    /// # Examples
    ///
    /// ```
    /// use list::v6::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        let mut some_prev: Option<Rc<RefCell<ListNode<T>>>> = None;
        let mut cur: Rc<RefCell<ListNode<T>>>;
        if let Some(ref head) = self.head {
            cur = Rc::clone(head);
        } else {
            // You can't pop the head of the list.
            return None;
        };

        while let Some(ref next) = Rc::clone(&cur).borrow().next {
            some_prev = Some(Rc::clone(&cur));
            cur = Rc::clone(next);
        }

        if let Some(prev) = some_prev {
            prev.borrow_mut().next = None;
        } else {
            self.head = None;
        }

        let last: ListNode<T> = Rc::try_unwrap(cur).ok().unwrap().into_inner();
        Some(last.value)
    }

    /// # Examples
    ///
    /// ```
    /// use list::v6::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> SinglyLinkedListIterator<T> {
        if let Some(ref head) = self.head {
            SinglyLinkedListIterator {
                cur: Some(Rc::downgrade(&Rc::clone(head)))
            }    
        } else {
            SinglyLinkedListIterator { cur: None }
        }
    }
}

impl<T:Clone> Iterator for SinglyLinkedListIterator<T> {
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
mod tests {
    use crate::v6::SinglyLinkedList;

    #[test]
    fn test_push_pop_1() {
        let mut list: SinglyLinkedList<u8> = Default::default();
        list.push_back(1);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        list.push_back(1);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_push_pop_2() {
        let mut list: SinglyLinkedList<&str> = Default::default();
        list.push_back("hello");
        list.push_back("world");
        assert_eq!(list.pop_back(), Some("world"));
        assert_eq!(list.pop_back(), Some("hello"));
        assert_eq!(list.pop_back(), None);
        list.push_back("hello");
        list.push_back("world");
        assert_eq!(list.pop_back(), Some("world"));
        assert_eq!(list.pop_back(), Some("hello"));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_iter_unwrap_failed() {
        let mut list: SinglyLinkedList<u8> = Default::default();
        list.push_back(1);
        list.push_back(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(iter.next(), None);

        list.push_back(2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_last_add() {
        let mut list: SinglyLinkedList<u8> = Default::default();
        list.push_back(1);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1));
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(iter.next(), None);
    }
}
