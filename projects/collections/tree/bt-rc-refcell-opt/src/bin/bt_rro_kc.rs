use bt_rc_refcell_opt::kc::BTree;

fn main() {
    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(6);
    tree.insert(5);
    println!("{:?}", tree);
}
