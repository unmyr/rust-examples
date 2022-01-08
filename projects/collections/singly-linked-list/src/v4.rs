use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ListNode<T: fmt::Debug> {
    value: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

#[derive(Default)]
pub struct SinglyLinkedList<T: fmt::Debug> {
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

impl<T: fmt::Debug> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
}

impl<T: fmt::Debug + Clone> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            head: None,
        }
    }

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

        let result: T;
        result = Rc::clone(&cur).borrow().value.clone();
        if let Some(prev) = some_prev {
            prev.borrow_mut().next = None;
        } else {
            self.head = None;
        }
        println!("pop_back(): END");
        Some(result)
    }
}

impl<T: fmt::Debug> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        println!("> Dropping: SinglyLinkedList");
    }
}

impl<T:fmt::Debug> Drop for ListNode<T> {
    fn drop(&mut self) {
        println!("> Dropping: {:?}", self.value);
    }
}
