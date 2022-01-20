use super::*;
#[test]
fn test_insert() {
    let mut tree: BTree<&str> = Default::default();
    tree.insert(&"E");
    tree.insert(&"A");
    tree.insert(&"S");
    tree.insert(&"Y");
    assert_eq!(
        format!("{:?}", tree),
        "BTree={TreeNode(Nil,\"A\",Nil), TreeNode(\"A\",\"E\",\"S\"), TreeNode(Nil,\"S\",\"Y\"), TreeNode(Nil,\"Y\",Nil)}"
    );
}
