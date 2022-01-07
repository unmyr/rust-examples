use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;

pub struct Node<T: Debug> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Default)]
pub struct List<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug> Node<T> {
    pub fn new(v: T) -> Node<T> {
        Node { value: v, next: None, prev: None }
    }
}

impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        println!("> Dropping: List");
    }
}

impl<T: Debug> Drop for Node<T> {
    fn drop(&mut self) {
        println!("> Dropping: Node {:?}", self.value);
    }
}

impl<T: Debug> List<T> {
    pub fn push_back(&mut self, v: T) {
        let node_new = Node::new(v);
        let mut cur: Rc<RefCell<Node<T>>>;
        if let Some(ref head) = self.head {
            cur = Rc::clone(head);
        } else {
            self.head = Some(Rc::new(RefCell::new(node_new)));
            return;
        };
        let mut some_prev: Option<Rc<RefCell<Node<T>>>> = None;

        while let Some(ref next) = Rc::clone(&cur).borrow().next {
            some_prev = Some(Rc::clone(&cur));
            cur = Rc::clone(next);
        }

        cur.borrow_mut().next = Some(
            Rc::new(RefCell::new(node_new))
        );
        if let Some(prev) = some_prev {
            cur.borrow_mut().prev = Some(Rc::clone(&prev));
        };
    }
}

impl<T: Debug> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.prev.as_ref(), self.next.as_ref()) {
            (None, None) => {
                write!(f, "Node({:?},Nil,Nil)", self.value)
            },
            (Some(_), None) => {
                write!(f, "Node({:?},*,Nil)", self.value)
            },
            (None, Some(next)) => {
                write!(f, "Node({:?},Nil,{})", self.value, next.borrow())
            },
            (Some(_), Some(next)) => {
                write!(f, "Node({:?},*,{})", self.value, next.borrow())
            }
        }
    }
}

impl<T: Debug> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            None => write!(f, "List(Nil)"),
            Some(ref head) => {
                write!(f, "List({})", head.borrow())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
