use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;

pub struct Node<T: Debug> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
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
        let mut node_new = Node::new(v);
        let mut cur: Rc<RefCell<Node<T>>>;
        if let Some(ref head) = self.head {
            cur = Rc::clone(head);
        } else {
            self.head = Some(Rc::new(RefCell::new(node_new)));
            return;
        };

        while let Some(ref next) = Rc::clone(&cur).borrow().next {
            cur = Rc::clone(next);
        }
        node_new.prev = Some(Rc::downgrade(&cur));

        cur.borrow_mut().next = Some(
            Rc::new(RefCell::new(node_new))
        );
    }
}

impl<T: Debug> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.prev.as_ref(), self.next.as_ref()) {
            (None, None) => {
                write!(f, "Node({:?}, Nil, Nil)", self.value)
            },
            (Some(prev), None) => {
                write!(
                    f, "Node({:?}, {:?}, Nil)",
                    self.value,
                    Rc::clone(&prev.upgrade().unwrap()).borrow().value
                )
            },
            (None, Some(next)) => {
                write!(
                    f, "Node({:?}, Nil, {:?}), {}",
                    self.value, next.borrow().value, next.borrow()
                )
            },
            (Some(prev), Some(next)) => {
                write!(
                    f, "Node({:?}, {:?}, {:?}), {}",
                    self.value,
                    Rc::clone(&prev.upgrade().unwrap()).borrow().value,
                    next.borrow().value,
                    next.borrow()
                )
            }
        }
    }
}

impl<T: Debug> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            None => write!(f, "List[]"),
            Some(ref head) => {
                write!(f, "List[{}]", head.borrow())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::List;

    #[test]
    fn test_push_back_u8() {
        let mut list: List<u8> = Default::default();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
    }
}
