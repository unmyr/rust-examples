use super::*;

#[test]
fn test_insert() {
    let tree: BTree<&str> = Default::default();
    tree.insert("E");
    tree.insert("A");
    tree.insert("S");
    tree.insert("Y");
    assert_eq!(
        format!("{:?}", &tree),
        "BTree={TreeNode(Nil,\"A\",Nil), TreeNode(\"A\",\"E\",\"S\"), TreeNode(Nil,\"S\",\"Y\"), TreeNode(Nil,\"Y\",Nil)}"
    );
}

#[test]
fn test_to_vec_in_order() {
    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    assert_eq!(tree.to_vec_in_order(), vec![4]);

    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    assert_eq!(tree.to_vec_in_order(), vec![2, 4]);

    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(6);
    assert_eq!(tree.to_vec_in_order(), vec![4, 6]);

    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(6);
    tree.insert(5);
    assert_eq!(
        tree.to_vec_in_order(),
        vec![1, 2, 4, 5, 6]
    );
}


#[test]
fn test_to_vec_pre_order() {
    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    assert_eq!(tree.to_vec_pre_order(), vec![4]);

    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    assert_eq!(tree.to_vec_pre_order(), vec![4, 2]);

    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(6);
    assert_eq!(tree.to_vec_pre_order(), vec![4, 6]);

    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(6);
    tree.insert(5);
    assert_eq!(
        tree.to_vec_pre_order(),
        vec![4, 2, 1, 6, 5]
    );
}

#[test]
fn test_iter_in_order() {
    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(6);
    tree.insert(5);
    assert_eq!(
        tree.iter_in_order().collect::<Vec<u8>>(),
        vec![1, 2, 4, 5, 6]
    );
}
