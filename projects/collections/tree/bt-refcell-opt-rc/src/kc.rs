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
                    left, left.key, self.key
                )
            },
        }
    }
}

#[derive(Default)]
pub struct BTree<K> {
    head: RefCell<Option<Rc<TreeNode<K>>>>,
}

impl<K: Ord> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_refcell_opt_rc::kc::BTree;
    /// let mut tree: BTree<&str> = Default::default();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// println!("{:?}", &tree);
    /// ```
    pub fn insert(&mut self, key: K) {
        if self.head.borrow().as_ref().is_none() {
            self.head.borrow_mut().replace(
                Rc::new(TreeNode::new(key))
            );
            return;
        }
        let cur_cell_ref = self.head.borrow();
        let cur_ref: &Rc<TreeNode<K>>;
        cur_ref = cur_cell_ref.as_ref().unwrap();

        let mut cur: Rc<TreeNode<K>> = Rc::clone(cur_ref);
        drop(cur_cell_ref);

        loop {
            let cur_cell_ref = match cur.key.cmp(&key) {
                Ordering::Greater => &cur.left,
                _ => &cur.right,
            };
            if cur_cell_ref.borrow().is_none() {
                cur_cell_ref.replace(
                    Some(Rc::new(TreeNode::new(key))
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

impl<K: Clone> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_refcell_opt_rc::kc::BTree;
    /// let mut tree: BTree<&str> = Default::default();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// assert_eq!(tree.to_vec_in_order(), vec!["A", "E", "S"]);
    /// ```
    pub fn to_vec_in_order(&self) -> Vec<K> {
        if self.head.borrow().is_none() {
            return Vec::new();
        }
        let cur_ref: &RefCell<Option<Rc<TreeNode<K>>>>;
        cur_ref = &self.head;

        let mut stack: Vec<Rc<TreeNode<K>>>;
        stack = Vec::new();
        let mut cur = Some(Rc::clone(cur_ref.borrow().as_ref().unwrap()));

        let mut results: Vec<K> = vec!();

        'outer: loop {
            // Traverse the subtree on the left while adding nodes to the stack.
            while cur.is_some() {
                if let Some(cur_rc_ref) = &cur {
                    stack.push(Rc::clone(cur_rc_ref));
                    match Rc::clone(cur_rc_ref).left.borrow().as_ref() {
                        Some(left_rc_ref) => {
                            cur = Some(Rc::clone(left_rc_ref));
                        },
                        None => {
                            cur = None;
                        }
                    }
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

                results.push(cur_right.key.clone());
                if cur_right.right.borrow().is_some() {
                    cur = Some(
                        Rc::clone(cur_right.right.borrow().as_ref().unwrap())
                    );
                    continue 'outer;
                }
            }
        }
        results
    }
}

impl<T: fmt::Debug> fmt::Debug for BTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head.borrow().as_ref() {
            None => write!(f, "BTree {{}}"),
            Some(head) => write!(f, "BTree={{{:?}}}", head),
        }
    }
}

#[cfg(test)]
mod tests;
