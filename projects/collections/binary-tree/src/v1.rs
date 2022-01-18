use std::fmt::{self};
use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
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

impl<'a, K: Ord> TreeNode<'a, K> {
    /// # Examples
    ///
    /// ```
    /// use binary_tree::v1::TreeNode;
    /// let mut node = TreeNode::new(&"E");
    /// node.insert(&"A");
    /// node.insert(&"S");
    /// println!("{:?}", &node);
    /// ```
    pub fn insert(&mut self, key_ref: &'a K) {
        let mut cur: Rc<RefCell<TreeNode<K>>>;
        let node_new: TreeNode<K> = TreeNode::<K>::new(key_ref);
        let cur_ref: &mut Option<Rc<RefCell<TreeNode<K>>>>;
        cur_ref = match self.key.cmp(key_ref) {
            Ordering::Greater => &mut self.left,
            _ => &mut self.right,
        };
        cur = match cur_ref {
            None => {
                cur_ref.replace(Rc::new(RefCell::new(node_new)));
                return
            },
            Some(ref cur_ref) => Rc::clone(cur_ref),
        };

        loop {
            let cur_ref: Ref<TreeNode<K>> = cur.borrow();
            let some_leaf: Option<Rc<RefCell<TreeNode<K>>>> = Ref::map(
                cur_ref,
                |n| {
                    match n.key.cmp(key_ref) {
                        Ordering::Greater => &n.left,
                    _ => &n.right,
                }
            }).clone();
            if some_leaf.is_none() {
                let mut some_leaf_ref_mut: RefMut<Option<_>> = RefMut::map(
                    cur.borrow_mut(),
                    |n| match n.key.cmp(key_ref) {
                        Ordering::Greater => &mut n.left,
                        _  => &mut n.right,
                    }
                );
                some_leaf_ref_mut.replace(Rc::new(RefCell::new(node_new)));
                return;
            }
            cur = Rc::clone(&some_leaf.unwrap());
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
                    left.borrow(), self.key, left.borrow().key
                )
            },
        }
    }
}

#[cfg(test)]
mod tests;
