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
        let cur_ref: &mut Option<Rc<RefCell<TreeNode<K>>>>;
        cur_ref = match self.key.cmp(&key) {
            Ordering::Less | Ordering::Equal => &mut self.right,
            Ordering::Greater => &mut self.left,
        };
        cur = match cur_ref {
            None => {
                *cur_ref = Some(Rc::new(RefCell::new(node_new)));
                return
            },
            Some(ref cur_ref) => Rc::clone(cur_ref),
        };

        loop {
            {
                let cur_ref_mut: RefMut<TreeNode<K>> = cur.borrow_mut();
                let mut some_leaf_ref_mut: RefMut<Option<_>> = RefMut::map(cur_ref_mut, |n|
                    if n.key.cmp(&key) == Ordering::Greater {
                        &mut n.left
                    } else {
                        &mut n.right
                    }
                );
                if some_leaf_ref_mut.is_none() {
                    *some_leaf_ref_mut = Some(Rc::new(RefCell::new(node_new)));
                    return;
                }
                drop(some_leaf_ref_mut);
            }

            let cur_ref: Ref<TreeNode<K>> = cur.borrow();
            let some_leaf: Option<Rc<RefCell<TreeNode<K>>>> = Ref::map(cur_ref, |n| {
                if n.key.cmp(&key) == Ordering::Greater {
                    &n.left
                } else {
                    &n.right
                }
            }).clone();
            cur = Rc::clone(&some_leaf.unwrap());
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
