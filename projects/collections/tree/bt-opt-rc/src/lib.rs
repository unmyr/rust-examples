use std::cmp::Ordering;
use std::fmt::{self};
use std::rc::Rc;

pub struct TreeNode<K> {
    key: K,
    left: Option<Rc<TreeNode<K>>>,
    right: Option<Rc<TreeNode<K>>>,
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
            }
            (Some(left), Some(right)) => {
                write!(
                    f,
                    "{:?}, TreeNode({:?},{:?},{:?}), {:?}",
                    left, left.key, self.key, right.key, right
                )
            }
            (None, Some(right)) => {
                write!(
                    f,
                    "TreeNode(Nil,{:?},{:?}), {:?}",
                    self.key, right.key, right
                )
            }
            (Some(left), None) => {
                write!(f, "{:?}, TreeNode({:?},{:?},Nil)", left, left.key, self.key)
            }
        }
    }
}

#[derive(Default)]
pub struct BTree<K> {
    head: Option<Rc<TreeNode<K>>>,
}

impl<K> BTree<K> {
    pub fn new() -> Self {
        BTree { head: None }
    }
}

impl<K: Ord> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_opt_rc::BTree;
    /// let mut tree: BTree<&str> = Default::default();
    /// tree.insert("E");
    /// tree.insert("A");
    /// tree.insert("S");
    /// println!("{:?}", &tree);
    /// ```
    pub fn insert(&mut self, key: K) {
        if self.head.is_none() {
            self.head.replace(Rc::new(TreeNode::new(key)));
            return;
        }
        let cur_ref: &mut Rc<TreeNode<K>>;
        cur_ref = self.head.as_mut().unwrap();

        let mut cur: Rc<TreeNode<K>>;
        cur = Rc::clone(cur_ref);

        loop {
            let some_next_rc_ref: &Option<Rc<TreeNode<K>>>;
            if cur.key.cmp(&key) == Ordering::Greater {
                some_next_rc_ref = &cur.left;
            } else {
                some_next_rc_ref = &cur.right;
            }
            if let Some(next_rc_ref) = some_next_rc_ref {
                cur = Rc::clone(next_rc_ref);
                continue;
            }

            assert_eq!(2, Rc::strong_count(&cur));
            unsafe {
                let ptr = Rc::into_raw(cur);
                Rc::decrement_strong_count(ptr);
                cur = Rc::from_raw(ptr);
            }
            assert_eq!(1, Rc::strong_count(&cur));
            if cur.key.cmp(&key) == Ordering::Greater {
                Rc::get_mut(&mut cur)
                    .unwrap()
                    .left
                    .replace(Rc::new(TreeNode::new(key)));
            } else {
                Rc::get_mut(&mut cur)
                    .unwrap()
                    .right
                    .replace(Rc::new(TreeNode::new(key)));
            }
            unsafe {
                let ptr = Rc::into_raw(cur);
                Rc::increment_strong_count(ptr);
                cur = Rc::from_raw(ptr);
            }
            assert_eq!(2, Rc::strong_count(&cur));
            return;
        }
    }
}

impl<K: Clone> BTree<K> {
    /// # Examples
    ///
    /// ```
    /// use bt_opt_rc::BTree;
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
        let cur_ref: &Option<Rc<TreeNode<K>>>;
        cur_ref = &self.head;

        let mut stack: Vec<Rc<TreeNode<K>>>;
        stack = Vec::new();
        let mut cur = Some(Rc::clone(cur_ref.as_ref().unwrap()));

        let mut results: Vec<K> = vec![];

        'outer: loop {
            // Traverse the subtree on the left while adding nodes to the stack.
            while cur.is_some() {
                stack.push(Rc::clone(cur.as_ref().unwrap()));
                if Rc::clone(cur.as_ref().unwrap()).left.is_none() {
                    cur = None;
                } else {
                    // cur = Rc::clone(cur.as_ref().unwrap()).left;
                    cur = Some(Rc::clone(
                        Rc::clone(cur.as_ref().unwrap()).left.as_ref().unwrap(),
                    ))
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
                if cur_right.right.is_some() {
                    cur = Some(Rc::clone(cur_right.right.as_ref().unwrap()));
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
            None => write!(f, "BTree={{}}"),
            Some(head) => write!(f, "BTree={{{:?}}}", head),
        }
    }
}

#[cfg(test)]
mod tests;
