use std::fmt;
use std::rc::Rc;
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
            None => write!(f, "ListNode({:?},Nil)", self.value),
            Some(ref next) => {
                write!(f, "ListNode({:?},{})", self.value, next.borrow())
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            None => write!(f, "SinglyLinkedList(Nil)"),
            Some(ref head) => {
                write!(f, "SinglyLinkedList({})", head.borrow())
            }
        }
    }
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            head: None,
        }
    }

    /// # Examples
    ///
    /// ```
    /// use list::v5::SinglyLinkedList;
    /// let mut list = SinglyLinkedList::new();
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
    /// use list::v5::SinglyLinkedList;
    /// let mut list = SinglyLinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        println!("pop_back(): BEGIN");
        let mut some_prev: Option<Rc<RefCell<ListNode<T>>>> = None;
        let mut cur: Rc<RefCell<ListNode<T>>>;
        if let Some(ref head) = self.head {
            cur = Rc::clone(head);
        } else {
            // You can't pop the head of the list.
            println!("pop_back(): END");
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
        println!("pop_back(): END");
        Some(last.value)
    }
}
