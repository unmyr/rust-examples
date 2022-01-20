use bt_rc_refcell_opt::kc::BTree;

fn main() {
    let tree: BTree<&str> = Default::default();
    tree.insert("E");
    tree.insert("A");
    tree.insert("S");
    tree.insert("Y");
    tree.insert("Z");
    println!("{:?}", tree);

    assert_eq!(
        tree.iter_in_order().collect::<Vec<&str>>(),
        vec!["A", "E", "S", "Y", "Z"]
    );
}
