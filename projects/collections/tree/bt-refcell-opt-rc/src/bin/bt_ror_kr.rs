use bt_refcell_opt_rc::kr::BTree;

fn main() {
    let mut tree: BTree<&str> = Default::default();
    tree.insert(&"E");
    tree.insert(&"A");
    tree.insert(&"S");
    tree.insert(&"Y");
    tree.insert(&"Z");
    dbg!(&tree);
}
