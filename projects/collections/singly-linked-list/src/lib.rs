use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SinglyLinkedList<T: Debug> {
    value: T,
    next: Option<Rc<RefCell<SinglyLinkedList<T>>>>,
}

impl<T: Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.next {
            None => write!(f, "SinglyLinkedList({:?}, Nil)", self.value),
            Some(ref next) => {
                write!(f, "SinglyLinkedList({:?}, {})", self.value, next.borrow())
            }
        }
    }
}

impl<T: Debug> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        println!("> Dropping: {:?}", self.value);
    }
}

impl<T: Clone + Debug> SinglyLinkedList<T> {
    pub fn new(v: T) -> SinglyLinkedList<T> {
        SinglyLinkedList {
            value: v,
            next: None,
        }
    }

    pub fn push_back(&mut self, v: T) {
        let node_new = SinglyLinkedList::new(v);
        let mut cur: Rc<RefCell<SinglyLinkedList<T>>>;
        if let Some(ref next) = self.next {
            cur = Rc::clone(next);
        } else {
            self.next = Some(Rc::new(RefCell::new(node_new)));
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
        let mut some_prev: Option<Rc<RefCell<SinglyLinkedList<T>>>> = None;
        let mut cur: Rc<RefCell<SinglyLinkedList<T>>>;
        if let Some(ref next) = self.next {
            cur = Rc::clone(next);
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
            self.next = None;
        }
        println!("pop_back(): END");
        return Some(result);
    }
}
