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
                    left, left.key, self.key
                )
            },
        }
    }
}

#[derive(Default)]
pub struct BTree<'a, K> {
    head: RefCell<Option<Rc<TreeNode<'a, K>>>>,
}

impl<'a, K: Ord> BTree<'a, K> {
    /// # Examples
    ///
    /// ```
    /// use bt_refcell_opt_rc::kr::BTree;
    /// let mut tree: BTree<&str> = Default::default();
    /// tree.insert(&"E");
    /// tree.insert(&"A");
    /// tree.insert(&"S");
    /// println!("{:?}", &tree);
    /// ```
    pub fn insert(&self, key_ref: &'a K) {
        if self.head.borrow().as_ref().is_none() {
            self.head.borrow_mut().replace(
                Rc::new(TreeNode::new(key_ref))
            );
            return;
        }
        let cur_cell_ref = self.head.borrow();
        let cur_ref: &Rc<TreeNode<K>>;
        cur_ref = cur_cell_ref.as_ref().unwrap();

        let mut cur: Rc<TreeNode<K>> = Rc::clone(cur_ref);
        drop(cur_cell_ref);

        loop {
            let cur_cell_ref = match cur.key.cmp(key_ref) {
                Ordering::Greater => &cur.left,
                _ => &cur.right,
            };
            if cur_cell_ref.borrow().is_none() {
                cur_cell_ref.replace(
                    Some(Rc::new(TreeNode::new(key_ref))
                ));
                return;
            }
            let work: Rc<TreeNode<K>> = Rc::clone(
                cur_cell_ref.borrow().as_ref().unwrap()
            );
            cur = work;
        }
    }
}

impl<'a, K: fmt::Debug> fmt::Debug for BTree<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.head.borrow().as_ref() {
            None => write!(f, "BTree {{}}"),
            Some(head) => write!(f, "BTree={{{:?}}}", head),
        }
    }
}

#[cfg(test)]
mod tests;
