use bt_opt_rc::BTree;

fn main() {
    //      4
    //    2   6
    //  1    5   
    let mut tree: BTree<u8> = Default::default();
    assert_eq!(format!("{:?}", tree), "BTree={}");

    tree.insert(4);
    assert_eq!(
        format!("{:?}", tree),
        "BTree={TreeNode(Nil,4,Nil)}"
    );

    tree.insert(2);
    assert_eq!(
        format!("{:?}", tree),
        "BTree={TreeNode(Nil,2,Nil), TreeNode(2,4,Nil)}"
    );

    tree.insert(1);
    assert_eq!(
        format!("{:?}", tree),
        "BTree={TreeNode(Nil,1,Nil), TreeNode(1,2,Nil), TreeNode(2,4,Nil)}"
    );

    tree.insert(6);
    assert_eq!(
        format!("{:?}", tree),
        "BTree={TreeNode(Nil,1,Nil), TreeNode(1,2,Nil), TreeNode(2,4,6), TreeNode(Nil,6,Nil)}"
    );

    tree.insert(5);
    assert_eq!(
        format!("{:?}", tree),
        "BTree={TreeNode(Nil,1,Nil), TreeNode(1,2,Nil), TreeNode(2,4,6), TreeNode(Nil,5,Nil), TreeNode(5,6,Nil)}"
    );
}
