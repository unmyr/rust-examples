use std::fmt::{self};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

pub struct TreeNode<K> {
    key: K,
    left: RefCell<Option<Rc<TreeNode<K>>>>,
    right: RefCell<Option<Rc<TreeNode<K>>>>,
}

impl<K> TreeNode<K> {
    pub fn new(key: K) -> Self {
        TreeNode {
            key,
            left: RefCell::new(None),
            right: RefCell::new(None),
        }
    }
}

impl<K: Clone + Ord> TreeNode<K> {
    /// # Examples
    ///
    /// ```
    /// use binary_tree::v2::TreeNode;
    /// let mut node = TreeNode::new("E");
    /// node.insert("A");
    /// node.insert("S");
    /// println!("{:?}", node);
    /// ```
    pub fn insert(&mut self, key: K) {
        let mut cur: Rc<TreeNode<K>>;
        let node_new: TreeNode<K> = TreeNode::<K>::new(key.clone());
        let cur_ref: &mut RefCell<Option<Rc<TreeNode<K>>>>;
        cur_ref = match self.key.cmp(&key) {
            Ordering::Less | Ordering::Equal => &mut self.right,
            Ordering::Greater => &mut self.left,
        };
        if cur_ref.borrow().is_none() {
            *cur_ref = RefCell::new(Some(Rc::new(node_new)));
            return;
        }
        cur = match cur_ref.borrow().clone() {
            Some(v) => v,
            None => return,
        };

        loop {
            let cur_ref_cell = if cur.key.cmp(&key) == Ordering::Greater {
                &cur.left
            } else {
                &cur.right
            };
            if cur_ref_cell.borrow().is_none() {
                cur_ref_cell.replace(Some(Rc::new(node_new)));
                return;
            }
            let work = match cur_ref_cell.borrow().clone() {
                Some(v) => Rc::clone(&v),
                None => return,
            };
            cur = work;
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.left.borrow().clone(), self.right.borrow().clone()) {
            (None, None) => {
                write!(f, "TreeNode(Nil,{:?},Nil)", self.key)
            },
            (Some(ref left), Some(ref right)) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},{:?}), {:?}",
                    left, left.key, self.key, right.key, right
                )
            },
            (None, Some(ref right)) => {
                write!(f,
                    "TreeNode(Nil,{:?},{:?}), {:?}",
                    self.key, right.key, right
                )
            },
            (Some(ref left), None) => {
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
