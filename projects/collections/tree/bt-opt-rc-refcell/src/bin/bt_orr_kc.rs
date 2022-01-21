use bt_opt_rc_refcell::kc::BTree;

fn main() {
    //      4
    //    2   6
    //  1    5   
    let mut tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(6);
    tree.insert(5);
    println!("{:?}", tree);
    dbg!(&tree);
}
