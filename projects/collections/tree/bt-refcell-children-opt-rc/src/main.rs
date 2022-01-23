
use bt_refcell_children_opt_rc::BTree;

fn main() {
    let mut tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(6);
    tree.insert(5);
    println!("{:?}", tree);
    dbg!(&tree);
}