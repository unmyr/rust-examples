use std::fmt::{self};
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::collections::VecDeque;

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

pub struct BTreeIterator<K> {
    results: Vec<K>,
    cur: Option<usize>,
}

impl<K: Clone> BTree<K> {
    pub fn iter(&self) -> BTreeIterator<K> {
        if self.head.borrow().is_none() {
            return BTreeIterator {
                results: Vec::<K>::new(),
                cur: None,
            };
        }
        let cur_ref: &Rc<RefCell<Option<TreeNode<K>>>>;
        cur_ref = &self.head;

        let mut queue: VecDeque<Rc<RefCell<Option<TreeNode<K>>>>> = VecDeque::new();
        let mut cur = Rc::clone(cur_ref);

        let mut results: Vec<K> = vec!();

        while !queue.is_empty() || cur.borrow().is_some() {
            if cur.borrow().is_some() {
                let cur_ref = cur.borrow();
                queue.push_back(Rc::clone(&cur));
                let next = Rc::clone(&cur_ref.as_ref().unwrap().left);
                drop(cur_ref);
                cur = next;
                continue;
            }
            cur = queue.pop_back().unwrap();
            let cur_ref = cur.borrow();
            results.push(
                cur_ref.as_ref().unwrap().key.clone()
            );
            let next = Rc::clone(&cur_ref.as_ref().unwrap().right);
            drop(cur_ref);
            cur = next;
        }
        BTreeIterator { results, cur: Some(0) }
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
