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
                    left.borrow(), left.borrow().key, self.key
                )
            },
        }
    }
}

#[derive(Default)]
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
    /// let mut tree: BTree<&str> = Default::default();
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
        cur_ref = self.head.as_ref().unwrap();

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

impl<K: Clone> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_opt_rc_refcell::kc::BTree;
    /// let mut tree: BTree<&str> = Default::default();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// assert_eq!(tree.to_vec_in_order(), vec!["A", "E", "S"]);
    /// ```
    pub fn to_vec_in_order(&self) -> Vec<K> {
        if self.head.is_none() {
            return Vec::new();
        }
        let cur_ref: &Option<Rc<RefCell<TreeNode<K>>>>;
        cur_ref = &self.head;

        let mut stack: Vec<Rc<RefCell<TreeNode<K>>>>;
        stack = Vec::new();
        let mut cur = Some(Rc::clone(cur_ref.as_ref().unwrap()));

        let mut results: Vec<K> = vec!();

        'outer: loop {
            // Traverse the subtree on the left while adding nodes to the stack.
            while cur.is_some() {
                stack.push(Rc::clone(cur.as_ref().unwrap()));
                if Rc::clone(cur.as_ref().unwrap()).borrow().left.is_none() {
                    cur = None;
                } else {
                    // cur = Rc::clone(cur.as_ref().unwrap()).borrow().left;
                    cur = Some(
                        Rc::clone(
                            Rc::clone(cur.as_ref().unwrap()).borrow().left.as_ref().unwrap()
                        )
                    )
                }
            }

            // It pops elements from the stack and continues to output,
            // returning to traversing the left side
            // if a node is found on the current right side.
            loop {
                let cur_right = match stack.pop() {
                    Some(cur_right) => cur_right,
                    None => break 'outer,
                };
                results.push(cur_right.borrow().key.clone());
                if cur_right.borrow().right.is_some() {
                    cur = Some(Rc::clone(cur_right.borrow().right.as_ref().unwrap()));
                    continue 'outer;
                }
            }
        }
        results
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
