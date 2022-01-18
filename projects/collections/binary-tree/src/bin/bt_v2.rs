use binary_tree::v2::TreeNode;

fn main() {
    let mut node = TreeNode::new(&"E");
    node.insert(&"A");
    node.insert(&"S");
    node.insert(&"Y");
    node.insert(&"Z");
    dbg!(&node);
}
