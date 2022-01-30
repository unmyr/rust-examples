use std::fmt::{self};
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::cmp::Ordering;

pub struct TreeNode<'a, K> {
    key: &'a K,
    left: Option<Rc<RefCell<TreeNode<'a, K>>>>,
    right: Option<Rc<RefCell<TreeNode<'a, K>>>>,
}

impl<'a, K> TreeNode<'a, K> {
    pub fn new(key: &'a K) -> Self {
        TreeNode {
            key,
            left: None,
            right: None,
        }
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for TreeNode<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.left.as_ref(), self.right.as_ref()) {
            (None, None) => {
                write!(f, "TreeNode(Nil,{:?},Nil)", self.key)
            },
            (Some(left), Some(right)) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},{:?}), {:?}",
                    left.borrow(), left.borrow().key, self.key, right.borrow().key, right.borrow()
                )
            },
            (None, Some(right)) => {
                write!(f,
                    "TreeNode(Nil,{:?},{:?}), {:?}",
                    self.key, right.borrow().key, right.borrow()
                )
            },
            (Some(left), None) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},Nil)",
                    left.borrow(), left.borrow().key, self.key
                )
            },
        }
    }
}

#[derive(Default)]
pub struct BTree<'a, K> {
    head: Option<Rc<RefCell<TreeNode<'a, K>>>>,
}

impl<'a, K> BTree<'a, K> {
    pub fn new() -> Self {
        BTree {
            head: None,
        }
    }
}

impl<'a, K: Ord> BTree<'a, K> {
    /// # Examples
    ///
    /// ```
    /// use bt_opt_rc_refcell::kr::BTree;
    /// let mut tree: BTree<&str> = Default::default();
    /// tree.insert(&"E");
    /// tree.insert(&"A");
    /// tree.insert(&"S");
    /// println!("{:?}", &tree);
    /// ```
    pub fn insert(&mut self, key_ref: &'a K) {
        if self.head.is_none() {
            self.head.replace(
                Rc::new(RefCell::new(TreeNode::new(key_ref)))
            );
            return;
        }
        let cur_ref: &Rc<RefCell<TreeNode<K>>>;
        cur_ref = self.head.as_ref().unwrap();

        let mut cur: Rc<RefCell<TreeNode<K>>>;
        cur = Rc::clone(cur_ref);

        loop {
            let cur_ref_mut: RefMut<TreeNode<K>> = cur.borrow_mut();
            let mut some_rc_ref_mut: RefMut<Option<Rc<RefCell<TreeNode<K>>>>>;

            if cur_ref_mut.key.cmp(key_ref) == Ordering::Greater {
                some_rc_ref_mut = RefMut::map(cur_ref_mut, |n| &mut n.left);
            } else {
                some_rc_ref_mut = RefMut::map(cur_ref_mut, |n| &mut n.right);
            }

            if some_rc_ref_mut.is_none() {
                some_rc_ref_mut.replace(
                    Rc::new(RefCell::new(TreeNode::new(key_ref)))
                );
                return;
            }

            let next_rc_ref = Rc::clone(some_rc_ref_mut.as_ref().unwrap());
            drop(some_rc_ref_mut);
            cur = next_rc_ref;
        }
    }
}

impl<'a, K: fmt::Debug> fmt::Debug for BTree<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.head {
            None => write!(f, "BTree {{}}"),
            Some(head) => write!(f, "BTree={{{:?}}}", head.borrow()),
        }
    }
}

#[cfg(test)]
mod tests;
