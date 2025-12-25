use std::cell::{RefCell, RefMut};
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::rc::Rc;

pub struct TreeNodes<K> {
    left: Option<Rc<TreeNode<K>>>,
    right: Option<Rc<TreeNode<K>>>,
}

pub struct TreeNode<K> {
    key: K,
    children: RefCell<TreeNodes<K>>,
}

impl<K> TreeNode<K> {
    pub fn new(key: K) -> Self {
        TreeNode {
            key,
            children: RefCell::new(TreeNodes {
                left: None,
                right: None,
            }),
        }
    }
}

impl<K: Debug> fmt::Debug for TreeNode<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (
            self.children.borrow().left.as_ref(),
            self.children.borrow().right.as_ref(),
        ) {
            (None, None) => {
                write!(f, "TreeNode(Nil,{:?},Nil)", self.key)
            }
            (Some(ref left), Some(ref right)) => {
                write!(
                    f,
                    "{:?}, TreeNode({:?},{:?},{:?}), {:?}",
                    left, left.key, self.key, right.key, right
                )
            }
            (None, Some(ref right)) => {
                write!(
                    f,
                    "TreeNode(Nil,{:?},{:?}), {:?}",
                    self.key, right.key, right
                )
            }
            (Some(ref left), None) => {
                write!(f, "{:?}, TreeNode({:?},{:?},Nil)", left, left.key, self.key)
            }
        }
    }
}

#[derive(Default)]
pub struct BTree<K> {
    head: Option<Rc<TreeNode<K>>>,
}

impl<K: Ord> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_refcell_children_opt_rc::BTree;
    /// let mut tree: BTree<&str> = Default::default();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// println!("{:?}", &tree);
    /// ```
    pub fn insert(&mut self, key: K) {
        let mut cur: Rc<TreeNode<K>> = match self.head.as_ref() {
            Some(head_rc_ref) => Rc::clone(head_rc_ref),
            None => {
                self.head.replace(Rc::new(TreeNode::new(key)));
                return;
            }
        };

        loop {
            let mut some_node_ref =
                RefMut::map(cur.children.borrow_mut(), |children_ref| {
                    match cur.key.cmp(&key) {
                        Ordering::Greater => &mut children_ref.left,
                        _ => &mut children_ref.right,
                    }
                });
            if some_node_ref.is_none() {
                some_node_ref.replace(Rc::new(TreeNode::new(key)));
                return;
            }
            let cur_work = Rc::clone(some_node_ref.as_ref().unwrap());
            drop(some_node_ref);
            cur = cur_work;
        }
    }
}

impl<K: Clone> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_refcell_children_opt_rc::BTree;
    /// let mut tree: BTree<&str> = Default::default();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// assert_eq!(tree.to_vec_in_order(), vec!["A", "E", "S"]);
    /// ```
    pub fn to_vec_in_order(&self) -> Vec<K> {
        let head_rc: Rc<TreeNode<K>> = match self.head.as_ref() {
            Some(head_rc_ref) => Rc::clone(head_rc_ref),
            None => {
                return Vec::new();
            }
        };

        let mut stack: Vec<Rc<TreeNode<K>>>;
        stack = Vec::new();

        let mut results: Vec<K> = vec![];

        let mut cur = Some(head_rc);
        'outer: loop {
            // Traverse the subtree on the left while adding nodes to the stack.
            while let Some(cur_rc) = cur {
                stack.push(Rc::clone(&cur_rc));
                match Rc::clone(&cur_rc).children.borrow().left.as_ref() {
                    Some(left_rc_ref) => {
                        cur = Some(Rc::clone(left_rc_ref));
                    }
                    None => {
                        cur = None;
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
                if let Some(right_rc_ref) = Rc::clone(&cur_right).children.borrow().right.as_ref() {
                    cur = Some(Rc::clone(right_rc_ref));
                    continue 'outer;
                }
            }
        }
        results
    }
}

impl<T: fmt::Debug> fmt::Debug for BTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head.as_ref() {
            None => write!(f, "BTree {{}}"),
            Some(head) => write!(f, "BTree={{{:?}}}", head),
        }
    }
}

#[cfg(test)]
mod tests;
