use std::fmt::{self};
use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
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

impl<T: fmt::Debug> fmt::Debug for TreeNode<T> {
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
                    left.borrow(), self.key, left.borrow().key
                )
            },
        }
    }
}

pub struct BTree<K> {
    head: Option<Rc<RefCell<TreeNode<K>>>>,
}

impl<K> BTree<K> {
    pub fn new() -> Self {
        BTree {
            head: None,
        }
    }
}

impl<K: Ord> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_opt_rc_refcell::kc::BTree;
    /// let mut tree = BTree::new();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// println!("{:?}", &tree);
    /// ```
    pub fn insert(&mut self, key: K) {
        if self.head.is_none() {
            self.head.replace(
                Rc::new(RefCell::new(TreeNode::new(key)))
            );
            return;
        }
        let cur_ref: &Rc<RefCell<TreeNode<K>>>;
        cur_ref = &self.head.as_ref().unwrap();

        let mut cur: Rc<RefCell<TreeNode<K>>>;
        cur = Rc::clone(cur_ref);

        loop {
            let cur_ref: Ref<TreeNode<K>> = cur.borrow();
            let some_leaf: Option<Rc<RefCell<TreeNode<K>>>> = Ref::map(
                cur_ref,
                |n| {
                    match n.key.cmp(&key) {
                        Ordering::Greater => &n.left,
                        _ => &n.right,
                    }
                }
            ).clone();
            if some_leaf.is_none() {
                let mut some_leaf_ref_mut: RefMut<Option<_>> = RefMut::map(
                    cur.borrow_mut(),
                    |n| match n.key.cmp(&key) {
                        Ordering::Greater => &mut n.left,
                        _  => &mut n.right,
                    }
                );
                some_leaf_ref_mut.replace(
                    Rc::new(RefCell::new(TreeNode::new(key)))
                );
                return;
            }
            cur = Rc::clone(&some_leaf.unwrap());
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for BTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.head {
            None => write!(f, "BTree {{}}"),
            Some(head) => write!(f, "BTree={{{:?}}}", head.borrow()),
        }
    }
}

#[cfg(test)]
mod tests;
