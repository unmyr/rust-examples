use bt_rc_refcell_opt::kc::BTree;

fn main() {
    let tree: BTree<&str> = Default::default();
    tree.insert("E");
    tree.insert("A");
    tree.insert("S");
    tree.insert("Y");
    tree.insert("Z");
    println!("{:?}", tree);
    for n in tree.iter() {
        println!("{}", n);
    }
}
