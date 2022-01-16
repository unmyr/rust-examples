use std::fmt::{self};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

pub struct TreeNode<K> {
    key: K,
    left: Option<Rc<RefCell<TreeNode<K>>>>,
    right: Option<Rc<RefCell<TreeNode<K>>>>,
}

impl<K> TreeNode<K> {
    pub fn new(key: K) -> Self {
        TreeNode {
            key,
            left: None,
            right: None,
        }
    }
}

impl<K: Clone + Ord> TreeNode<K> {
    /// # Examples
    ///
    /// ```
    /// use binary_tree::TreeNode;
    /// let mut node = TreeNode::new("E");
    /// node.insert("A");
    /// node.insert("S");
    /// println!("{:?}", node);
    /// ```
    pub fn insert(&mut self, key: K) {
        let mut cur: Rc<RefCell<TreeNode<K>>>;
        let node_new: TreeNode<K> = TreeNode::<K>::new(key.clone());
        cur = match self.key.cmp(&key) {
            Ordering::Less | Ordering::Equal => {
                match self.right {
                    None => {
                        self.right = Some(Rc::new(RefCell::new(node_new)));
                        return
                    },
                    Some(ref right) => Rc::clone(right),
                }
            },
            Ordering::Greater => {
                match self.left {
                    None => {
                        self.left = Some(Rc::new(RefCell::new(node_new)));
                        return
                    },
                    Some(ref left) => Rc::clone(left),
                }
            },
        };

        loop {
            enum TreeArm {Left, Right}
            let hand: TreeArm;
            hand = match cur.borrow().key.cmp(&key) {
                Ordering::Less | Ordering::Equal => TreeArm::Right,
                Ordering::Greater => TreeArm::Left,
            };
            cur = match hand {
                TreeArm::Left => {
                    if cur.borrow().left.is_none() {
                        cur.borrow_mut().left = Some(Rc::new(RefCell::new(node_new)));
                        return;
                    }
                    match cur.borrow().left {
                        None => return (),
                        Some(ref next) => Rc::clone(next)
                    }
                },
                TreeArm::Right => {
                    if cur.borrow().right.is_none() {
                        cur.borrow_mut().right = Some(Rc::new(RefCell::new(node_new)));
                        return;
                    }
                    match cur.borrow().right {
                        None => return (),
                        Some(ref next) => Rc::clone(next)
                    }
                }
            };
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.left.as_ref(), self.right.as_ref()) {
            (None, None) => {
                write!(f, "TreeNode(Nil,{:?},Nil)", self.key)
            },
            (Some(ref left), Some(ref right)) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},{:?}), {:?}",
                    left.borrow(), left.borrow().key, self.key, right.borrow().key, right.borrow()
                )
            },
            (None, Some(ref right)) => {
                write!(f,
                    "TreeNode(Nil,{:?},{:?}), {:?}",
                    self.key, right.borrow().key, right.borrow()
                )
            },
            (Some(ref left), None) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},Nil)",
                    left.borrow(), self.key, left.borrow().key
                )
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut node = TreeNode::new("E");
        node.insert("A");
        node.insert("S");
        node.insert("Y");
        assert_eq!(
            format!("{:?}", node),
            "TreeNode(Nil,\"A\",Nil), TreeNode(\"A\",\"E\",\"S\"), TreeNode(Nil,\"S\",\"Y\"), TreeNode(Nil,\"Y\",Nil)"
        );
    }
}
