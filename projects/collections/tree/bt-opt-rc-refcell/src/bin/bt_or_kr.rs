use bt_opt_rc_refcell::kr::TreeNode;

fn main() {
    let mut node = TreeNode::new(&"E");
    node.insert(&"A");
    node.insert(&"S");
    node.insert(&"Y");
    node.insert(&"Z");
    dbg!(&node);
}
