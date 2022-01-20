use bt_opt_rc_refcell::kc::BTree;

fn main() {
    let mut tree = BTree::new();
    tree.insert("E");
    tree.insert("A");
    tree.insert("S");
    tree.insert("Y");
    tree.insert("Z");
    dbg!(&tree);
}
