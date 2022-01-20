use std::fmt::{self};
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::cmp::Ordering;

pub struct TreeNode<K> {
    key: K,
    left: Rc<RefCell<Option<TreeNode<K>>>>,
    right: Rc<RefCell<Option<TreeNode<K>>>>,
}

impl<K> TreeNode<K> {
    pub fn new(key: K) -> Self {
        TreeNode {
            key,
            left: Rc::new(RefCell::new(None)),
            right: Rc::new(RefCell::new(None)),
        }
    }
}

impl<K: Ord> TreeNode<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_rc_refcell_opt::kc::TreeNode;
    /// let node = TreeNode::new("E");
    /// node.insert("A");
    /// node.insert("S");
    /// println!("{:?}", &node);
    /// ```
    pub fn insert(&self, key: K) {
        let cur_ref: &Rc<RefCell<Option<TreeNode<K>>>>;
        cur_ref = match self.key.cmp(&key) {
            Ordering::Greater => &self.left,
            _ => &self.right,
        };

        let mut cur: Rc<RefCell<Option<TreeNode<K>>>>;
        cur = Rc::clone(cur_ref);

        loop {
            if cur.borrow().is_none() {
                cur.borrow_mut().replace(TreeNode::<K>::new(key));
                return;
            }

            let cur_ref: Ref<Option<TreeNode<K>>> = cur.borrow();
            let next = match cur_ref.as_ref().unwrap().key.cmp(&key) {
                Ordering::Greater => &cur_ref.as_ref().unwrap().left,
                _ => &cur_ref.as_ref().unwrap().right,
            }.clone();
            drop(cur_ref);

            cur = next;
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.left.borrow().as_ref(), self.right.borrow().as_ref()) {
            (None, None) => {
                write!(f, "TreeNode(Nil,{:?},Nil)", self.key)
            },
            (Some(left), Some(right)) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},{:?}), {:?}",
                    left, left.key, self.key, right.key, right
                )
            },
            (None, Some(right)) => {
                write!(f,
                    "TreeNode(Nil,{:?},{:?}), {:?}",
                    self.key, right.key, right
                )
            },
            (Some(left), None) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},Nil)",
                    left, self.key, left.key
                )
            },
        }
    }
}

#[cfg(test)]
mod tests;
