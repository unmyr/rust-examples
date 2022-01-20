use super::*;
#[test]
fn test_insert() {
    let node: BTree<&str> = BTree::new();
    node.insert("E");
    node.insert("A");
    node.insert("S");
    node.insert("Y");
    assert_eq!(
        format!("{:?}", &node),
        "BTree={TreeNode(Nil,\"A\",Nil), TreeNode(\"A\",\"E\",\"S\"), TreeNode(Nil,\"S\",\"Y\"), TreeNode(Nil,\"Y\",Nil)}"
    );
}
