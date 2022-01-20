use bt_rc_refcell_opt::kc::TreeNode;

fn main() {
    let node = TreeNode::new("E");
    node.insert("A");
    node.insert("S");
    node.insert("Y");
    node.insert("Z");
    println!("{:?}", node);
}
