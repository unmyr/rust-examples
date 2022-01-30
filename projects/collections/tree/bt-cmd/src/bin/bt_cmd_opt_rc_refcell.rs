use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;

pub struct TreeNode<K> {
    key: K,
    left: Option<Rc<RefCell<TreeNode<K>>>>,
    right: Option<Rc<RefCell<TreeNode<K>>>>,
}

impl<K> From<K> for TreeNode<K> {
    fn from(key: K) -> Self {
        TreeNode { key, left: None, right: None }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

pub struct BTree<K> {
    head: Option<Rc<RefCell<TreeNode<K>>>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for BTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.head {
            None => write!(f, "BTree={{}}"),
            Some(head) => write!(f, "BTree={{{:?}}}", head.borrow()),
        }
    }
}

fn main() {
    let tree_nodes = TreeNode {
        key: "m",
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::from("z")))),
    };
    let tree = BTree { head: Some(Rc::new(RefCell::new(tree_nodes))) };
    let key = "d";

    let cur_rc_ref: &Rc<RefCell<TreeNode<&str>>>;
    cur_rc_ref = tree.head.as_ref().unwrap();
    let cur_ref: RefMut<TreeNode<&str>> = cur_rc_ref.borrow_mut();

    let mut next_node: RefMut<Option<Rc<RefCell<TreeNode<&str>>>>>;
    if cur_ref.key.cmp(key) > Ordering::Less {
        next_node = RefMut::map(cur_ref, |n| &mut n.left);
    } else {
        next_node = RefMut::map(cur_ref, |n| &mut n.right);
    }
    if next_node.is_none() {
        next_node.replace(
            Rc::new(RefCell::new(TreeNode::from("d")))
        );
    }
    drop(next_node);

    let cur_left_ref = Ref::map(cur_rc_ref.borrow(), |n| &n.left);
    let cur_right_ref = Ref::map(cur_rc_ref.borrow(), |n| &n.right);
    assert_eq!("d", cur_left_ref.clone().as_ref().unwrap().borrow().key);
    assert_eq!("z", cur_right_ref.clone().as_ref().unwrap().borrow().key);
    dbg!(&tree);
}