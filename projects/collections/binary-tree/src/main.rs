use binary_tree::TreeNode;

fn main() {
    let mut node = TreeNode::new("E");
    node.insert("A");
    node.insert("S");
    node.insert("Y");
    println!("{:?}", node);
}
