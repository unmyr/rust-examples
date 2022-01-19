use super::*;
#[test]
fn test_insert() {
    let mut node = TreeNode::new(&"E");
    node.insert(&"A");
    node.insert(&"S");
    node.insert(&"Y");
    assert_eq!(
        format!("{:?}", node),
        "TreeNode(Nil,\"A\",Nil), TreeNode(\"A\",\"E\",\"S\"), TreeNode(Nil,\"S\",\"Y\"), TreeNode(Nil,\"Y\",Nil)"
    );
}
