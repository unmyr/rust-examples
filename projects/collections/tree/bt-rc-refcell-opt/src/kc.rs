use std::fmt::{self};
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::cmp::Ordering;

pub struct TreeNode<K> {
    key: K,
    left: Rc<RefCell<Option<TreeNode<K>>>>,
    right: Rc<RefCell<Option<TreeNode<K>>>>,
}

impl<K> TreeNode<K> {
    pub fn new(key: K) -> Self {
        TreeNode {
            key,
            left: Rc::new(RefCell::new(None)),
            right: Rc::new(RefCell::new(None)),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.left.borrow().as_ref(), self.right.borrow().as_ref()) {
            (None, None) => {
                write!(f, "TreeNode(Nil,{:?},Nil)", self.key)
            },
            (Some(left), Some(right)) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},{:?}), {:?}",
                    left, left.key, self.key, right.key, right
                )
            },
            (None, Some(right)) => {
                write!(f,
                    "TreeNode(Nil,{:?},{:?}), {:?}",
                    self.key, right.key, right
                )
            },
            (Some(left), None) => {
                write!(f,
                    "{:?}, TreeNode({:?},{:?},Nil)",
                    left, self.key, left.key
                )
            },
        }
    }
}

#[derive(Default)]
pub struct BTree<K> {
    head: Rc<RefCell<Option<TreeNode<K>>>>,
}

impl<K: Ord> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_rc_refcell_opt::kc::BTree;
    /// let tree: BTree<&str> = Default::default();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// println!("{:?}", &tree);
    /// ```
    pub fn insert(&self, key: K) {
        let cur_ref: &Rc<RefCell<Option<TreeNode<K>>>>;
        cur_ref = &self.head;

        let mut cur: Rc<RefCell<Option<TreeNode<K>>>>;
        cur = Rc::clone(cur_ref);

        loop {
            if cur.borrow().is_none() {
                cur.borrow_mut().replace(TreeNode::<K>::new(key));
                return;
            }

            let cur_ref: Ref<Option<TreeNode<K>>> = cur.borrow();
            let next = match cur_ref.as_ref().unwrap().key.cmp(&key) {
                Ordering::Greater => &cur_ref.as_ref().unwrap().left,
                _ => &cur_ref.as_ref().unwrap().right,
            }.clone();
            drop(cur_ref);

            cur = next;
        }
    }
}

impl<K: Clone> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_rc_refcell_opt::kc::BTree;
    /// let tree: BTree<&str> = Default::default();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// assert_eq!(tree.to_vec_in_order(), vec!["A", "E", "S"]);
    /// ```
    pub fn to_vec_in_order(&self) -> Vec<K> {
        if self.head.borrow().is_none() {
            return Vec::new();
        }
        let cur_ref: &Rc<RefCell<Option<TreeNode<K>>>>;
        cur_ref = &self.head;

        let mut stack: Vec<Rc<RefCell<Option<TreeNode<K>>>>>;
        stack = Vec::new();
        let mut cur = Rc::clone(cur_ref);

        let mut results: Vec<K> = vec!();

        'outer: loop {
            // Traverse the subtree on the left while adding nodes to the stack.
            while let Some(node) = Rc::clone(&cur).borrow().as_ref() {
                stack.push(Rc::clone(&cur));
                cur = Rc::clone(&node.left);
            }

            // It pops elements from the stack and continues to output,
            // returning to traversing the left side
            // if a node is found on the current right side.
            loop {
                let cur_right = match stack.pop() {
                    Some(cur_right) => cur_right,
                    None => break 'outer,
                };

                if let Some(node) = cur_right.borrow().as_ref() {
                    results.push(node.key.clone());
                    if node.right.borrow().is_some() {
                        cur = Rc::clone(&node.right);
                        continue 'outer;
                    }
                };
            }
        }
        results
    }
}

pub struct BTreeIterator<K> {
    results: Vec<K>,
    cur: Option<usize>,
}

impl<K: Clone> BTree<K> {
    pub fn iter_in_order(&self) -> BTreeIterator<K> {
        BTreeIterator {
            results: self.to_vec_in_order(),
            cur: Some(0)
        }
    }
}

impl<K:Clone> Iterator for BTreeIterator<K> {
    type Item = K;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur.as_ref()?;
        let mut i = self.cur.unwrap();
        if i >= self.results.len() {
            self.cur = None;
            return None;
        }
        let cur_key = &self.results[i];
        i += 1;
        self.cur.replace(i);
        Some(cur_key.clone())
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
