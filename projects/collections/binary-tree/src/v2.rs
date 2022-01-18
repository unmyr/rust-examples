use std::fmt::{self};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

pub struct TreeNode<'a, K> {
    key: &'a K,
    left: RefCell<Option<Rc<TreeNode<'a, K>>>>,
    right: RefCell<Option<Rc<TreeNode<'a, K>>>>,
}

impl<'a, K> TreeNode<'a, K> {
    pub fn new(key: &'a K) -> Self {
        TreeNode {
            key,
            left: RefCell::new(None),
            right: RefCell::new(None),
        }
    }
}

impl<'a, K: Ord> TreeNode<'a, K> {
    /// # Examples
    ///
    /// ```
    /// use binary_tree::v2::TreeNode;
    /// let mut node = TreeNode::new(&"E");
    /// node.insert(&"A");
    /// node.insert(&"S");
    /// println!("{:?}", &node);
    /// ```
    pub fn insert(&mut self, key_ref: &'a K) {
        let mut cur: Rc<TreeNode<K>>;
        let node_new: TreeNode<K> = TreeNode::<K>::new(key_ref);
        let cur_cell_ref: &RefCell<Option<Rc<TreeNode<K>>>>;
        cur_cell_ref = match self.key.cmp(key_ref) {
            Ordering::Greater => &mut self.left,
            _ => &mut self.right,
        };
        if cur_cell_ref.borrow().is_none() {
            cur_cell_ref.replace(Some(Rc::new(node_new)));
            return;
        }
        cur = Rc::clone(
            cur_cell_ref.borrow().as_ref().unwrap()
        );

        loop {
            let cur_cell_ref = match cur.key.cmp(key_ref) {
                Ordering::Greater => &cur.left,
                _ => &cur.right,
            };
            if cur_cell_ref.borrow().is_none() {
                cur_cell_ref.replace(Some(Rc::new(node_new)));
                return;
            }
            let work: Rc<TreeNode<K>> = Rc::clone(
                cur_cell_ref.borrow().as_ref().unwrap()
            );
            cur = work;
        }
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for TreeNode<'a, T> {
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
